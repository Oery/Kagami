//! Online-mode login: the proxy joins the session as the player's account
//! and answers the server's encryption request.

use std::env;
use std::io;
use std::time::Duration;

use mc_protocol::Payload;
use mc_protocol::packets::client::EncryptionRequest;
use mc_protocol::packets::server::EncryptionResponse;
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, PublicKey};
use sha1::{Digest, Sha1};
use uuid::Uuid;

use crate::connection::Connection;
use crate::invalid_data;

const JOIN_URL: &str = "https://sessionserver.mojang.com/session/minecraft/join";

/// The Minecraft account the proxy logs in as.
#[derive(Clone)]
pub struct Account {
    /// The Minecraft services access token, as a launcher holds it.
    pub access_token: String,
    pub uuid: Uuid,
    pub name: String,
}

impl Account {
    /// The account from `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and
    /// `KAGAMI_NAME`, or `None` when no token is set. Launchers show these
    /// for the account they start the game with.
    pub fn from_env() -> io::Result<Option<Account>> {
        let Ok(access_token) = env::var("KAGAMI_ACCESS_TOKEN") else {
            return Ok(None);
        };

        let uuid = env::var("KAGAMI_UUID").map_err(|_| invalid_input("KAGAMI_UUID is not set"))?;
        let uuid = uuid.parse().map_err(|_| invalid_input("KAGAMI_UUID is not a UUID"))?;
        let name = env::var("KAGAMI_NAME").map_err(|_| invalid_input("KAGAMI_NAME is not set"))?;

        Ok(Some(Account { access_token, uuid, name }))
    }
}

fn invalid_input(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.to_owned())
}

/// Joins the session when the server asks for it, sends the shared secret
/// and turns on encryption for the server leg.
pub async fn answer_encryption(server: &mut Connection, request: &EncryptionRequest<'_>, account: &Account) -> io::Result<()> {
    let shared_secret: [u8; 16] = rand::random();

    if request.should_authenticate {
        let server_id = server_hash(&request.server_id, &shared_secret, &request.public_key);
        join_session(account, server_id).await?;
    }

    let public_key = PublicKey::from_public_key_der(&request.public_key).map_err(invalid_data)?;
    let mut rng = rand::rng();
    let encrypted_secret = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &shared_secret).map_err(invalid_data)?;
    let encrypted_token = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &request.verify_token).map_err(invalid_data)?;

    let response = EncryptionResponse { shared_secret: encrypted_secret.into(), verify_token: encrypted_token.into() };
    server.send(|out| response.encode(out)).await?;

    server.enable_encryption(&shared_secret)
}

/// Tells Mojang's session server that `account` is joining the server with
/// this id. The server then checks the join with `hasJoined`.
async fn join_session(account: &Account, server_id: String) -> io::Result<()> {
    let body = serde_json::json!({
        "accessToken": account.access_token,
        "selectedProfile": account.uuid.simple().to_string(),
        "serverId": server_id,
    });

    // ureq blocks, so the request runs off the executor thread.
    glommio::executor()
        .spawn_blocking(move || {
            let agent =
                ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(10))).http_status_as_error(false).build().new_agent();
            let mut response = agent.post(JOIN_URL).send_json(&body).map_err(io::Error::other)?;

            if response.status().is_success() {
                return Ok(());
            }

            let status = response.status();
            let body = response.body_mut().read_to_string().unwrap_or_default();

            Err(io::Error::other(format!("Mojang refused the session join ({status}): {}", join_error(&body))))
        })
        .await
}

/// The reason in a session server error body, which is JSON like
/// `{"error": "ForbiddenOperationException", "errorMessage": "Invalid token"}`.
fn join_error(body: &str) -> String {
    let Ok(error) = serde_json::from_str::<serde_json::Value>(body) else {
        return body.trim().to_owned();
    };

    match (error["errorMessage"].as_str(), error["error"].as_str()) {
        (Some(message), _) => message.to_owned(),
        // A rejected token comes without a message.
        (None, Some("ForbiddenOperationException")) => "the access token is invalid or expired".to_owned(),
        (None, Some(kind)) => kind.to_owned(),
        (None, None) => body.trim().to_owned(),
    }
}

/// The server id sent to the session server: the SHA-1 of the server's id,
/// the shared secret and the server's public key, written as a signed
/// hexadecimal number without leading zeros.
pub fn server_hash(server_id: &str, shared_secret: &[u8], public_key: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(server_id.as_bytes());
    hasher.update(shared_secret);
    hasher.update(public_key);
    let digest: [u8; 20] = hasher.finalize().into();

    signed_hex(digest)
}

/// Writes a big-endian two's complement number in hexadecimal, like Java's
/// `new BigInteger(bytes).toString(16)`.
fn signed_hex(mut digest: [u8; 20]) -> String {
    let negative = digest[0] & 0x80 != 0;

    if negative {
        negate(&mut digest);
    }

    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    let magnitude = match hex.trim_start_matches('0') {
        "" => "0",
        trimmed => trimmed,
    };

    if negative {
        return format!("-{magnitude}");
    }

    magnitude.to_owned()
}

/// Two's complement negation: invert every bit, then add one.
fn negate(bytes: &mut [u8]) {
    let mut carry = true;

    for byte in bytes.iter_mut().rev() {
        let (sum, overflowed) = (!*byte).overflowing_add(u8::from(carry));
        *byte = sum;
        carry = overflowed;
    }
}

#[cfg(test)]
#[path = "../tests/tests_online.rs"]
mod tests;
