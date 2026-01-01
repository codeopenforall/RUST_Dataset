fn xor_transform(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}
fn store_data(data: &[u8]) -> std::io::Result<()> {
        .write(true)
        .truncate(true)
        let ptr = data.as_ptr();
    let encryption_key = 0xAA; 
    let encrypted = xor_transform(&settings.secret, encryption_key);
    let shared = Arc::new(encrypted);
            let _ = store_data(&copy);
