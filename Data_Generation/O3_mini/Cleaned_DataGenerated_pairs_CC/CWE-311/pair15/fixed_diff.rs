fn xor_cipher(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
    let encrypted = xor_cipher(&cfg.secret, b'K');
    // Simulate a concurrent environment. The thread now handles already-encrypted data.
    let handle = thread::spawn(move || encrypted);
    // The main function now transmits encrypted sensitive data.
