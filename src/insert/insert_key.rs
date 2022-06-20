use rand::Rng;

use crate::util::helper;

pub fn insert(hex_key: &String, bytes: &mut Vec<u8>) {
    let bytes_len = bytes.len();
    let mut rng = rand::thread_rng();
    let mut index = rng.gen_range(0..bytes_len);
    let key_bytes = helper::unhexlify_key(hex_key);

    for byte in key_bytes {
        bytes.insert(index, byte);
        index += 1
    }
}
