use super::*;

#[test]
fn server_hashes_are_signed_hexadecimal_like_java() {
    // The known digests of these names, one positive and two negative.
    assert_eq!(server_hash("Notch", &[], &[]), "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48");
    assert_eq!(server_hash("jeb_", &[], &[]), "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1");
    assert_eq!(server_hash("simon", &[], &[]), "88e16a1019277b15d58faf0541e11910eb756f6");
}

#[test]
fn negation_carries_across_bytes() {
    let mut bytes = [0x00, 0x00];
    negate(&mut bytes);
    assert_eq!(bytes, [0x00, 0x00]);

    let mut bytes = [0xff, 0x00];
    negate(&mut bytes);
    assert_eq!(bytes, [0x01, 0x00]);
}

#[test]
fn session_errors_are_reduced_to_their_reason() {
    let forbidden = r#"{ "error" : "ForbiddenOperationException", "path" : "/session/minecraft/join" }"#;
    assert_eq!(join_error(forbidden), "the access token is invalid or expired");
    assert_eq!(join_error(r#"{"error": "X", "errorMessage": "Invalid token"}"#), "Invalid token");
    assert_eq!(join_error(" Bad Gateway \n"), "Bad Gateway");
}
