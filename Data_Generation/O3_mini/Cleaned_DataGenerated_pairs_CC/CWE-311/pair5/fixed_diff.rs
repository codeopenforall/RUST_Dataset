fn encrypt_secret(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| b ^ key)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}
    let key: u8 = 0xAA; 
    let encrypted = encrypt_secret(&cfg.secret, key);
        SENT_DATA = Some(encrypted);
