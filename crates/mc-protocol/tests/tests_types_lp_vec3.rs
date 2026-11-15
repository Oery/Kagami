use super::*;
#[test]
fn vectors_match_official_26_3_codec_and_bound_quantization() {
    // Bytes captured from net.minecraft.network.LpVec3.write, including
    // continuation scale and Java's NaN/infinity sanitization.
    for (values, expected) in [
        ([0.0, 0.0, 0.0], vec![0]),
        ([0.0, -0.0392, 0.0], vec![0xf9, 0xff, 0x7f, 0xfe, 0xf5, 0xf5]),
        ([0.0, -1.95, 0.0], vec![0xfa, 0xff, 0x7f, 0xfe, 0x06, 0x69]),
        ([0.25, -4.5, 1.2], vec![0x95, 0x19, 0x9e, 0xb6, 0x19, 0x9a, 1]),
        (
            [f64::NAN, f64::INFINITY, f64::NEG_INFINITY],
            vec![0xff, 0xff, 0, 1, 0xff, 0xf9, 0xff, 0xff, 0xff, 0xff, 0x0f],
        ),
    ] {
        let vector = LpVec3(values);
        let mut bytes = Vec::new();
        vector.encode(&mut bytes).unwrap();
        assert_eq!(bytes, expected);
        assert_eq!(bytes.len(), vector.body_len());
        let mut input = bytes.as_slice();
        let decoded = LpVec3::read(&mut input).unwrap();
        assert!(input.is_empty());
        let scale = values.into_iter().filter(|v| v.is_finite()).map(f64::abs).fold(1.0, f64::max).ceil();
        if values.iter().all(|v| v.is_finite()) {
            for (actual, wanted) in decoded.0.into_iter().zip(values) {
                assert!((actual - wanted).abs() <= scale / 32766.0 + 1e-12);
            }
        }
        for length in 0..bytes.len() {
            assert!(LpVec3::read(&mut &bytes[..length]).is_err());
        }
    }
}
