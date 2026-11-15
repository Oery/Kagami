use super::*;
use crate::State;
use crate::packets::{Context, Packets};

fn seen() -> LastSeen {
    LastSeen { count: 0, acknowledged: [0; 3], checksum: 1 }
}

#[test]
fn chat_decodes_signed_and_unsigned_and_rejects_truncated_metadata() {
    for signature in [None, Some(Signature(&[7; 256]))] {
        let packet = ChatMessage { message: ChatText("Hello, 世界!"), timestamp: 12, salt: 42, signature, seen: seen() };
        let mut body = Vec::new();
        packet.encode_payload(&mut body).unwrap();
        assert_eq!(packet.payload_len().unwrap(), body.len());
        let mut input = body.as_slice();
        let decoded = ChatMessage::decode_payload(&mut input).unwrap();
        assert_eq!(decoded.message.0, "Hello, 世界!");
        assert_eq!(decoded.signature.is_some(), packet.signature.is_some());
        assert!(input.is_empty());
        for length in 0..body.len() {
            assert!(ChatMessage::decode_payload(&mut &body[..length]).is_err());
        }
        let mut frame = vec![9];
        frame.extend_from_slice(&body);
        let context = Context { state: State::Play, ..Default::default() };
        assert!(matches!(Packets::read(&mut frame.as_slice(), &context).unwrap(), Packets::ChatMessage(_)));
    }
}

#[test]
fn signed_command_consumes_fixed_signatures_and_checksum() {
    let packet = SignedChatCommand {
        command: "msg Alex hello there",
        timestamp: 1,
        salt: 2,
        signatures: ArgumentSignatures(vec![("message", Signature(&[3; 256]))]),
        seen: seen(),
    };
    let mut body = Vec::new();
    packet.encode_payload(&mut body).unwrap();
    assert_eq!(packet.payload_len().unwrap(), body.len());
    let mut input = body.as_slice();
    let decoded = SignedChatCommand::decode_payload(&mut input).unwrap();
    assert_eq!(decoded.command, packet.command);
    assert_eq!(decoded.signatures.0[0].1.0, &[3; 256]);
    assert!(input.is_empty());
    assert!(ArgumentSignatures::read(&mut [9].as_slice()).is_err());
    assert!(ArgumentSignatures::read(&mut [1, 49].as_slice()).is_err());
}

#[test]
fn chat_text_limit_counts_utf16_and_checks_lengths_before_reading() {
    for (text, valid) in [("a".repeat(256), true), ("😀".repeat(128), true), ("😀".repeat(129), false)] {
        let mut bytes = Vec::new();
        text.as_str().encode(&mut bytes).unwrap();
        assert_eq!(ChatText::read(&mut bytes.as_slice()).is_ok(), valid);
    }
    assert!(ChatText::read(&mut [0x81, 0x06].as_slice()).is_err());
}
