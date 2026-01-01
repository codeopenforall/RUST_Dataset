fn encode(data: &str, salt: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ salt).collect()
}
unsafe fn deliver(data: &[u8]) -> Result<(), std::io::Error> {
    stream.write_all(data)?;
    let salt: u8 = 0xAA;
    let protected = encode(&settings.key, salt);
    let shared_data = Arc::new(protected);
    let data_copy = Arc::clone(&shared_data);
            match deliver(&data_copy) {
                Ok(_) => println!("Transmission finished (secure)"),
