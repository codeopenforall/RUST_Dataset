fn basic_encrypt(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}
    println!("Transmitting data: {:?}", data);
    let key: u8 = 0xAA; 
        let encrypted = basic_encrypt(&shared_config.sensitive, key);
        send(&encrypted)
