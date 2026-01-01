const XOR_KEY: u8 = 0xAA;
fn xor_encrypt(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ XOR_KEY).collect()
}
fn to_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}
        self.secret.as_bytes().to_vec()
        let encrypted = xor_encrypt(&bytes);
        to_hex(&encrypted)
