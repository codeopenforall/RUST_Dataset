const XOR_KEY: u8 = 0xAA;
fn xor_cipher(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ XOR_KEY).collect()
}
        let plaintext = self.api_key.as_bytes();
        xor_cipher(plaintext)
    let encrypted_result = shared.lock().unwrap();
    println!("Transmitted encrypted data: {:?}", encrypted_result);
