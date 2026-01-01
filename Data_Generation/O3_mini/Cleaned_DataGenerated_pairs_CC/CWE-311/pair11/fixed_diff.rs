const KEY: u8 = 0xAB; 
fn encrypt_data(secret: &String) -> String {
    secret
        .bytes()
        .map(|b| format!("{:02x}", b ^ KEY))
        .collect::<String>()
    let handle = std::thread::spawn(move || encrypt_data(&config.secret));
