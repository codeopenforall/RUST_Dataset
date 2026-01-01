fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
            let _ = slice;
    let key: u8 = 0xAA; 
    let encrypted = xor_encrypt(data.secret.as_bytes(), key);
    file.write_all(&encrypted).expect("Failed to write file");
