fn encrypt(data: &[u8]) -> Vec<u8> {
    let key: u8 = 0xAA; 
    data.iter().map(|b| b ^ key).collect()
}
        let raw = data.secret.as_bytes();
        let cipher = encrypt(raw);
        let mut file = File::create("output.txt")
            .expect("Unable to create output file");
        file.write_all(&cipher)
            .expect("Failed to write encrypted data to file");
