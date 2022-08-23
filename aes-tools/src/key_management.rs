/// Save an AES file, encoded in hex.
pub fn save_to_file<P: AsRef<std::path::Path>>(path: P, data: Vec<u8>) {
    std::fs::write(path, hex::encode(&data)).unwrap();
}

/// Retrieve an AES key from a hex-encoded file.
pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Vec<u8> {
    let data = std::fs::read_to_string(path).expect("unable to load key from file");
    hex::decode(data).expect("was unable to convert from hex")
}
