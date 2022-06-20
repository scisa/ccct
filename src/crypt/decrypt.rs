use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};

use crate::util::error_messages::ERROR_DECRYPTION_FAILED;
use crate::util::exit_codes::EXIT_DECRYPTION_FAILED;
use crate::util::global_constants::NONCE;
use crate::util::helper;

#[derive(Debug)]
pub struct Decrypted {
    pub plaintext: String,
}

impl Decrypted {
    pub fn decrypt(hex_key: &String, enc_data: &mut Vec<u8>) -> Decrypted {
        let byte_key = helper::unhexlify_key(hex_key);
        let key = Key::from_slice(&byte_key[..]);
        let cipher = Aes256Gcm::new(key);
        let plaintext = match cipher.decrypt(Nonce::from_slice(NONCE.as_bytes()), &enc_data[..]) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}: {}", ERROR_DECRYPTION_FAILED, e);
                std::process::exit(EXIT_DECRYPTION_FAILED);
            }
        };

        Self {
            plaintext: String::from(String::from_utf8_lossy(&plaintext)),
        }
    }
}
