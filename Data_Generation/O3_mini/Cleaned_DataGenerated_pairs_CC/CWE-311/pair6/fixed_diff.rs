fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
fn xor_decrypt(data: &[u8], key: u8) -> Vec<u8> {
    xor_encrypt(data, key)
}
    xor_encrypt(secret_bytes, 0xAA)
        let encrypted_data = transmit(&config_clone);
        let decrypted = xor_decrypt(&encrypted_data, 0xAA);
        println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));
