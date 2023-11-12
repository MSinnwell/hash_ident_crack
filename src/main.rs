mod hash_algorithms;
mod crack_hash;

use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::hash_algorithms::*;
use crate::crack_hash::*;


fn crack_hash(hash_type: &str, hash: String) -> Option<String> {
    match hash_type {
        "md5" => Some(crack_hashes(&hash, HashType::MD5)),
        "sha1" => Some(crack_hashes(&hash, HashType::SHA1)),
        "sha256" => Some(crack_hashes(&hash, HashType::SHA256)),
        "sha512" => Some(crack_hashes(&hash, HashType::SHA512)),
        "crc32" => Some(crack_hashes(&hash, HashType::CRC32)),
        _ => None,
    }
}



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No hash provided");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No hash provided!"));
    }

    let hash = &args[1];
    let hash_type = identify_hash_type(&hash);

    if hash_type.is_none() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Hash could not be identified!"));
    }

    let file = File::open("wordlist")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line?;
        if let Some(hex) = crack_hash(hash_type.as_deref().unwrap(), word.clone()) {
            if hex == *hash {
                println!("The original text was: {:?}", word);
                return Ok(());
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "No match found in wordlist"))
}