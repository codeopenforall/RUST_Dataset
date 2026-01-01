fn encrypt(input: &str) -> String {
    let key = b'K';
    input.bytes().map(|b| (b ^ key) as char).collect()
}
/// Processes sensitive data by encrypting it before transmission.
/// Returns the encrypted data.
        // Encrypt the secret before sending.
        let encrypted = encrypt(&arc_clone.secret);
        tx.send(encrypted).unwrap();
