use sha2::{Digest, Sha512, Sha256};
use sha1::{Sha1};
use md5::compute;
use crc32fast::Hasher;

pub enum HashType {
    SHA512,
    SHA256,
    SHA1,
    MD5,
    CRC32,
}

pub fn crack_hashes(string: &str, hash_type: HashType) -> String {
    match hash_type {
        HashType::SHA512 => {
            let mut hasher = Sha512::new();
            hasher.update(string.as_bytes());
            hasher.finalize().iter().map(|byte| format!("{:02x}", byte)).collect()
        },
        HashType::SHA256 => {
            let mut hasher = Sha256::new();
            hasher.update(string.as_bytes());
            hasher.finalize().iter().map(|byte| format!("{:02x}", byte)).collect()
        },
        HashType::SHA1 => {
            let mut hasher = Sha1::new();
            hasher.update(string.as_bytes());
            hasher.finalize().iter().map(|byte| format!("{:02x}", byte)).collect()
        },
        HashType::MD5 => {
            let digest = compute(string.as_bytes());
            format!("{:x}", digest)
        },
        HashType::CRC32 => {
            let mut hasher = Hasher::new();
            hasher.update(&string.as_bytes());
            let crc = hasher.finalize();
            format!("{:x}", crc)
        },
    }
}