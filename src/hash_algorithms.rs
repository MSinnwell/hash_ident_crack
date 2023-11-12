pub fn identify_hash_type(hash: &str) -> Option<String> {
    let hash_lengths = vec![
        (32, "md5"),
        (40, "sha1"),
        (64, "sha256"),
        (128, "sha512"),
        (8, "crc32"),
    ];

    for (length, hash_type) in hash_lengths {
        if hash.len() == length
            && !hash.chars().all(|c| c.is_digit(10))
            && !hash.chars().all(char::is_alphabetic)
            && hash.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Some(hash_type.to_string());
        }
    }

    None
}