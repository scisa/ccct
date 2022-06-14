use std::fs;

use crate::util::global_constants::INSERT_KEY_FILE_ENDING;
use crate::util::error_messages::ERROR_WRITE_INSERT_KEY_FILE_FAILED;
use crate::util::exit_codes::EXIT_WRITE_INSERT_KEY_FILE_FAILED;

pub fn write_insert_key_file(file_name: &String, data: &Vec<u8>) {
    if let Err(e) = fs::write(format!("{}{}", file_name, INSERT_KEY_FILE_ENDING), data) {
        eprintln!("{}: {}", ERROR_WRITE_INSERT_KEY_FILE_FAILED, e);
        std::process::exit(EXIT_WRITE_INSERT_KEY_FILE_FAILED);
    }
}