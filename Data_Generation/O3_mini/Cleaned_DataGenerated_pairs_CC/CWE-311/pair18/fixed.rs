use std::fs::File;
use std::io::Write;
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
struct Settings {
    secret: String,
}
impl Settings {
    fn secure(&self) -> Vec<u8> {
        let key: u8 = 0xAA;
        xor_encrypt(self.secret.as_bytes(), key)
    }
}
fn store_data(data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("config.txt")?;
    file.write_all(data)?;
    Ok(())
}
fn run_main() {
    let settings = Settings { secret: "SuperSecret!".to_string() };
    let encrypted_data = settings.secure();
    if let Err(e) = store_data(&encrypted_data) {
        eprintln!("Error storing configuration: {}", e);
    }
}
fn main() {
    run_main();
}
