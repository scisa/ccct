use regex::Regex;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use hex;

use crate::util::global_constants::{NONCE, SECRET_KEY};
use crate::util::exit_codes::EXIT_ENCRYPTION_FAILED;
use crate::util::error_messages::ERROR_ENCRYPTION_FAILED;

#[derive(Debug)]
pub struct Encrypted {
    pub key_hex_string: String,
    pub enc_data: Vec<u8>,
}


impl Encrypted {
    pub fn encrypt(data: &String) -> Self {
        let mut encryted_data = Self::encrypt_data(&data);
        
        let key_hex_string = Self::extract_key_from_string(&encryted_data.key_hex_string);   
        encryted_data.key_hex_string = key_hex_string;

        encryted_data
    }
        
    fn encrypt_data(data: &String) -> Self {
        let key = Key::from_slice(SECRET_KEY.as_bytes());
        let key_string: String = format!("{:#?}", key);
        let cipher = Aes256Gcm::new(key);
    
        let mut buffer: Vec<u8> = vec![0; data.as_bytes().len() + 16]; // Buffer needs 16-bytes overhead for GCM tag
        buffer.extend_from_slice(data.as_bytes());
    
        let ciphertext = match cipher.encrypt(Nonce::from_slice(NONCE.as_bytes()), data.as_bytes().as_ref()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}: {}", ERROR_ENCRYPTION_FAILED, e);
                std::process::exit(EXIT_ENCRYPTION_FAILED);
            }  
        };
    
        Self {
            key_hex_string: key_string,
            enc_data: ciphertext,
        }
    }

    fn extract_key_from_string(key_string: &String) -> String {
        let mut key: Vec<u8> = Vec::new();
        let re = Regex::new(r"\d+").unwrap();
        for cap in re.captures_iter(key_string) {
            key.push(std::str::FromStr::from_str(&cap[0]).unwrap());
        }
    
        hex::encode(key)
    }
}