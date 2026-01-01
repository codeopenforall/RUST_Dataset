}
fn encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}
fn decrypt(data: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    String::from_utf8_lossy(&decrypted).into_owned()
    let key: u8 = 0xAA;
    let encrypted_data = encrypt(&plain_secret, key);
    file.write_all(&encrypted_data)
    let decrypted = decrypt(&encrypted_data, key);
    assert_eq!(decrypted, plain_secret, "Decryption failed");
