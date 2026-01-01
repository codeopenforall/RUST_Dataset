use std::fs::File;
use std::io::Write;

// A simple symmetric transformation using XOR with a fixed key for demonstration purposes.
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}

struct Settings {
    secret: String,
}

impl Settings {
    // Securely convert the secret using a simple XOR "encryption".
    fn secure(&self) -> Vec<u8> {
        let key: u8 = 0xAA;
        xor_encrypt(self.secret.as_bytes(), key)
    }
}

// A function that writes encrypted data to disk.
fn store_data(data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("config.txt")?;
    file.write_all(data)?;
    Ok(())
}

fn run_main() {
    // Create settings with the sensitive secret.
    let settings = Settings { secret: "SuperSecret!".to_string() };

    // Encrypt the secret before storing.
    let encrypted_data = settings.secure();

    // Write the encrypted data to disk.
    if let Err(e) = store_data(&encrypted_data) {
        eprintln!("Error storing configuration: {}", e);
    }
}

fn main() {
    run_main();
}