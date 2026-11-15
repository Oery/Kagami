use super::*;
use crate::Payload;

fn body<'p>(packet: &impl Payload<'p>, id: i32) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();
    let mut input = bytes.as_slice();
    assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, input.len());
    assert_eq!(VarInt::read(&mut input).unwrap().0, id);
    assert_eq!(packet.payload_len().unwrap(), input.len());

    input.to_vec()
}

#[test]
fn objective_modes_and_number_formats_have_exact_conditional_fields() {
    let properties = || ObjectiveProperties { title: Nbt::Str("t".into()), render: ScoreRender::Integer, number_format: None };
    assert_eq!(
        body(&SetObjective { name: "o".into(), action: ObjectiveAction::Create(properties()) }, 0x6C),
        [1, b'o', 0, 8, 0, 1, b't', 0, 0]
    );
    assert_eq!(
        body(&SetObjective { name: "o".into(), action: ObjectiveAction::Update(properties()) }, 0x6C),
        [1, b'o', 2, 8, 0, 1, b't', 0, 0]
    );
    assert_eq!(body(&SetObjective { name: "o".into(), action: ObjectiveAction::Remove }, 0x6C), [1, b'o', 1]);
    let formats = [
        (ScoreNumberFormat::Blank, vec![0]),
        (
            ScoreNumberFormat::Styled(Nbt::Compound(vec![("bold".into(), Nbt::Bool(true))])),
            vec![1, 10, 1, 0, 4, b'b', b'o', b'l', b'd', 1, 0],
        ),
        (ScoreNumberFormat::Fixed(Nbt::Str("x".into())), vec![2, 8, 0, 1, b'x']),
    ];

    for (format, expected) in formats {
        let packet = SetObjective {
            name: "o".into(),
            action: ObjectiveAction::Create(ObjectiveProperties {
                render: ScoreRender::Hearts,
                number_format: Some(format),
                ..properties()
            }),
        };
        let mut bytes = vec![1, b'o', 0, 8, 0, 1, b't', 1, 1];
        bytes.extend(expected);
        assert_eq!(body(&packet, 0x6C), bytes);
    }
}

#[test]
fn scores_encode_signed_values_optional_labels_and_scoped_resets() {
    let score = |display, number_format| SetScore { holder: "p".into(), objective: "o".into(), value: -1, display, number_format };
    assert_eq!(body(&score(None, None), 0x70), [1, b'p', 1, b'o', 255, 255, 255, 255, 15, 0, 0]);
    assert_eq!(
        body(&score(Some(Nbt::Str("d".into())), Some(ScoreNumberFormat::Blank)), 0x70),
        [1, b'p', 1, b'o', 255, 255, 255, 255, 15, 1, 8, 0, 1, b'd', 1, 0]
    );
    assert_eq!(body(&ResetScore { holder: "p".into(), objective: Some("o".into()) }, 0x50), [1, b'p', 1, 1, b'o']);
    assert_eq!(body(&ResetScore { holder: "p".into(), objective: None }, 0x50), [1, b'p', 0]);
}

#[test]
fn display_slots_and_clear_use_varint_and_empty_name() {
    for slot in 0..=2 {
        assert_eq!(body(&SetDisplayObjective { slot, name: "o".into() }, 0x64), [slot as u8, 1, b'o']);
        assert_eq!(body(&SetDisplayObjective { slot, name: "".into() }, 0x64), [slot as u8, 0]);
    }

    assert!(ScoreRender::read(&mut [2].as_slice()).is_err());
    assert!(ObjectiveAction::read(&mut [3].as_slice()).is_err());
    assert!(ScoreNumberFormat::read(&mut [3].as_slice()).is_err());
}
