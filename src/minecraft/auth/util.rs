use base64::{Engine as _, engine::general_purpose};
use sha2::{Digest, Sha256, digest::generic_array::GenericArray};
use urlencoding;

pub fn generate_string(length: usize) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let chars: Vec<char> = charset.chars().collect();
    let mut result = String::with_capacity(length);

    unsafe {
        for _ in 0..length {
            result.push(*chars.get_unchecked(fastrand::usize(0..chars.len())))
        }
    }

    result
}

pub fn encode_base64(input: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(input)
}

pub fn encode_url(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

pub fn hash_sha256(input: String) -> GenericArray<u8, sha2::digest::generic_array::typenum::U32> {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    hash
}
