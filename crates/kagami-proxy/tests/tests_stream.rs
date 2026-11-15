use futures_lite::future::block_on;

use super::*;
use crate::cipher::ciphers;

#[test]
fn encryption_continues_across_writes() {
    let mut written = Vec::new();
    let (encryptor, mut decryptor) = ciphers(&[9; 16]);
    let mut writer = Writer::new(&mut written);
    writer.enable_encryption(encryptor);

    let mut first = *b"hello ";
    let mut second = *b"world";
    block_on(writer.write_all_in_place(&mut first)).unwrap();
    block_on(writer.write_all_in_place(&mut second)).unwrap();

    assert_ne!(written, b"hello world");
    decryptor.decrypt(&mut written);
    assert_eq!(written, b"hello world");
}
