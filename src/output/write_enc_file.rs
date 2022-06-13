use std::fs;

use crate::util::global_constants::ENC_FILE_ENDING;
use crate::util::error_messages::ERROR_WRITE_ENC_FILE_FAILED;
use crate::util::exit_codes::EXIT_WRITE_ENC_FILE_FAILED;

pub fn write_encrypted_file(file_name: &String, enc_data: &Vec<u8>) {
    if let Err(e) = fs::write(format!("{}{}", file_name, ENC_FILE_ENDING), enc_data) {
        eprintln!("{}: {}", ERROR_WRITE_ENC_FILE_FAILED, e);
        std::process::exit(EXIT_WRITE_ENC_FILE_FAILED);
    }
}