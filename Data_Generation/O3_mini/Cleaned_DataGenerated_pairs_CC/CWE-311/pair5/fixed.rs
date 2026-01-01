use std::sync::Arc;
use std::thread;
struct Config {
    secret: String,
}
static mut SENT_DATA: Option<String> = None;
fn encrypt_secret(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| b ^ key)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}
fn send_data(cfg: Arc<Config>) {
    let key: u8 = 0xAA; 
    let encrypted = encrypt_secret(&cfg.secret, key);
    unsafe {
        SENT_DATA = Some(encrypted);
    }
}
pub fn simulate_send() -> String {
    let cfg = Arc::new(Config { secret: "supersecret".to_string() });
    let cfg_clone = Arc::clone(&cfg);
    let handle = thread::spawn(move || {
         send_data(cfg_clone);
    });
    handle.join().unwrap();
    unsafe {
         SENT_DATA.clone().unwrap_or_default()
    }
}
fn main() {
    let transmitted = simulate_send();
    println!("Transmitted data: {}", transmitted);
}
