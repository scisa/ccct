use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use hex;

use crate::util::global_constants::NONCE;
use crate::util::error_messages::{ERROR_UNHEXLIFY_KEY_FAILED, ERROR_DECRYPTION_FAILED};
use crate::util::exit_codes::{EXIT_UNHEXLIFYING_KEY_FAILED, EXIT_DECRYPTION_FAILED};


#[derive(Debug)]
pub struct Decrypted {
    pub plaintext: String,
}

impl Decrypted {
    pub fn decrypt(hex_key: &String, enc_data: &mut Vec<u8>) -> Decrypted {
        let byte_key = Self::unhexlify_key(hex_key);
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

    fn unhexlify_key(hex_key: &String) -> Vec<u8> {
        let byte_key = match hex::decode(hex_key) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("{}: {}", ERROR_UNHEXLIFY_KEY_FAILED, e);
                std::process::exit(EXIT_UNHEXLIFYING_KEY_FAILED);
            }
        };

        byte_key
    }
}