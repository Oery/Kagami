//! AES-128-CFB8, the stream cipher of online-mode connections. Both
//! directions are keyed by the shared secret, which is also the IV.
//!
//! CFB8 encrypts one AES block per byte. Encrypting is serial: each block
//! starts from the ciphertext byte before it. Decrypting is not, since the
//! whole ciphertext is known, so [`Decryptor`] encrypts many blocks at once,
//! which AES-NI pipelines.

use aes::cipher::{BlockCipherEncrypt, KeyInit};
use aes::{Aes128, Block};
use cfb8::cipher::KeyIvInit;

/// Encrypts in place, continuing the stream across calls.
pub type Encryptor = cfb8::Encryptor<Aes128>;

const BLOCK_LEN: usize = 16;

/// Bytes decrypted per batch of blocks.
const BATCH_LEN: usize = 256;

/// Decrypts in place, continuing the stream across calls.
pub struct Decryptor {
    cipher: Aes128,
    /// The last sixteen ciphertext bytes, from which the next byte's
    /// keystream is encrypted.
    shift_register: [u8; BLOCK_LEN],
}

impl Decryptor {
    pub fn new(key: &[u8; BLOCK_LEN], iv: &[u8; BLOCK_LEN]) -> Self {
        Self { cipher: Aes128::new(key.into()), shift_register: *iv }
    }

    pub fn decrypt(&mut self, bytes: &mut [u8]) {
        for batch in bytes.chunks_mut(BATCH_LEN) {
            self.decrypt_batch(batch);
        }
    }

    /// Byte `i` of `batch` is XORed with the first byte of the block
    /// encrypted from the sixteen ciphertext bytes before it.
    fn decrypt_batch(&mut self, batch: &mut [u8]) {
        // The register, then the batch's ciphertext, which decrypting in
        // place is about to overwrite.
        let mut ciphertext = [0; BLOCK_LEN + BATCH_LEN];
        ciphertext[..BLOCK_LEN].copy_from_slice(&self.shift_register);
        ciphertext[BLOCK_LEN..BLOCK_LEN + batch.len()].copy_from_slice(batch);

        let mut blocks = [Block::default(); BATCH_LEN];
        let blocks = &mut blocks[..batch.len()];

        for (index, block) in blocks.iter_mut().enumerate() {
            block.copy_from_slice(&ciphertext[index..index + BLOCK_LEN]);
        }

        self.cipher.encrypt_blocks(blocks);

        for (byte, keystream) in batch.iter_mut().zip(blocks.iter()) {
            *byte ^= keystream[0];
        }

        self.shift_register.copy_from_slice(&ciphertext[batch.len()..batch.len() + BLOCK_LEN]);
    }
}

pub fn ciphers(shared_secret: &[u8; BLOCK_LEN]) -> (Encryptor, Decryptor) {
    let encryptor = Encryptor::new(shared_secret.into(), shared_secret.into());
    let decryptor = Decryptor::new(shared_secret, shared_secret);

    (encryptor, decryptor)
}

#[cfg(test)]
#[path = "../tests/tests_cipher.rs"]
mod tests;
