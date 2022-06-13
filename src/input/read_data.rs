use std::fs::{self, File};
use std::io::{self, BufReader, Read};

use crate::util::global_constants::DEFAULT_INPUT_TEXT;


pub fn get_data(file_path: &String) -> String {
    let data = match fs::read_to_string(file_path) {
        Ok(d) => d,
        Err(_) => String::from(DEFAULT_INPUT_TEXT),
    };

    data
}

pub fn read_byte_data(file_path: &String) -> Vec<u8> {
    let bytes = match read_bytes_to_vector(&file_path) {
        Ok(b) => b,
        Err(_) => Vec::new(),
    };

    bytes
}

fn read_bytes_to_vector(file_path: &str) -> io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}