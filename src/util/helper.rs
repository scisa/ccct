use crate::util::error_messages::ERROR_UNHEXLIFY_KEY_FAILED;
use crate::util::exit_codes::EXIT_UNHEXLIFYING_KEY_FAILED;


pub fn unhexlify_key(hex_key: &String) -> Vec<u8> {
    let byte_key = match hex::decode(hex_key) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("{}: {}", ERROR_UNHEXLIFY_KEY_FAILED, e);
            std::process::exit(EXIT_UNHEXLIFYING_KEY_FAILED);
        }
    };

    byte_key
}