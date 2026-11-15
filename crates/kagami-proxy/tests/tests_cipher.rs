use super::*;

#[test]
fn batched_decryption_matches_the_reference_across_uneven_calls() {
    let secret = [3; 16];
    let plaintext: Vec<u8> = (0..2000u32).map(|index| (index * 31 % 251) as u8).collect();

    let mut bytes = plaintext.clone();
    Encryptor::new(&secret.into(), &secret.into()).encrypt(&mut bytes);

    let mut reference = bytes.clone();
    cfb8::Decryptor::<Aes128>::new(&secret.into(), &secret.into()).decrypt(&mut reference);
    assert_eq!(reference, plaintext);

    // Calls shorter than a block, across batches, and longer than a batch.
    let mut decryptor = Decryptor::new(&secret, &secret);
    let mut rest = bytes.as_mut_slice();

    for len in [1, 15, 17, 300, 255, 1000] {
        let (call, after) = rest.split_at_mut(len);
        decryptor.decrypt(call);
        rest = after;
    }

    decryptor.decrypt(rest);
    assert_eq!(bytes, plaintext);
}
