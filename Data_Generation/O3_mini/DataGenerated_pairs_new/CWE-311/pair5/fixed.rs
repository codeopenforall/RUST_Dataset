//////////////////////////////////////////////
// Secure implementation: sensitive data is encrypted before transmission.
//////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

// Structure holding configuration with a sensitive field.
struct Config {
    secret: String,
}

// Global static mutable variable to simulate network transmission storage.
// Using unsafe global state solely for legacy compatibility.
static mut SENT_DATA: Option<String> = None;

// Encryption function: performs a simple XOR-based encryption and returns a hex-encoded string.
// Note: In production, use well-vetted cryptographic libraries.
fn encrypt_secret(data: &str, key: u8) -> String {
    data.bytes()
        .map(|b| b ^ key)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// Function to simulate sending sensitive data over a secure channel.
// It encrypts the secret before storing it.
fn send_data(cfg: Arc<Config>) {
    let key: u8 = 0xAA; // Fixed key for demonstration.
    let encrypted = encrypt_secret(&cfg.secret, key);
    // Using unsafe block to mimic legacy interactions.
    unsafe {
        SENT_DATA = Some(encrypted);
    }
}

// Public API to trigger the transmission process with encryption.
// It spawns a thread which calls the secure send_data function.
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