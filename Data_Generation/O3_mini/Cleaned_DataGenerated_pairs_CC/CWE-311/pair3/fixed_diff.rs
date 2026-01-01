fn secure_transform(data: &str) -> String {
    let xor_key: u8 = 0xAA;
    data.bytes()
        .map(|b| format!("{:02X}", b ^ xor_key))
        .collect()
}
        secure_transform(&config_clone.key)
