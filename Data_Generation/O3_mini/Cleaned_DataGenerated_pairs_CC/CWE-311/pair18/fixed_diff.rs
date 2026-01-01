fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
    fn secure(&self) -> Vec<u8> {
        let key: u8 = 0xAA;
        xor_encrypt(self.secret.as_bytes(), key)
    file.write_all(data)?;
    let encrypted_data = settings.secure();
    if let Err(e) = store_data(&encrypted_data) {
