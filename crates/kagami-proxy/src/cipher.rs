//! AES-128-CFB8, the stream cipher of online-mode connections. Both
//! directions are keyed by the shared secret, which is also the IV.

use aes::Aes128;
use cfb8::cipher::KeyIvInit;

/// Encrypts in place, continuing the stream across calls.
pub type Encryptor = cfb8::Encryptor<Aes128>;

/// Decrypts in place, continuing the stream across calls.
pub type Decryptor = cfb8::Decryptor<Aes128>;

pub fn ciphers(shared_secret: &[u8; 16]) -> (Encryptor, Decryptor) {
    let encryptor = Encryptor::new(shared_secret.into(), shared_secret.into());
    let decryptor = Decryptor::new(shared_secret.into(), shared_secret.into());

    (encryptor, decryptor)
}
