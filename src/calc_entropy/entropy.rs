use crate::util::helper;

pub fn entropy(hex_key: &String) -> f32 {
    let key_bytes = helper::unhexlify_key(hex_key);
    entropy::shannon_entropy(&key_bytes)
}