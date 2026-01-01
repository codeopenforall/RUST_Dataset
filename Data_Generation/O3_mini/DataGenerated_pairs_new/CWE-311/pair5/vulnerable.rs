//////////////////////////////////////////////
// Insecure implementation: sensitive data is transmitted as clear text.
//////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

// Structure holding configuration with a sensitive field.
struct Config {
    secret: String,
}

// Global static mutable variable to simulate network transmission storage.
// Using unsafe global state for demonstration purposes.
static mut SENT_DATA: Option<String> = None;

// Function to simulate sending sensitive data over an insecure channel.
// Uses an unsafe block to directly clone and store the secret without encryption.
fn send_data(cfg: Arc<Config>) {
    let secret_ptr: *const String = &cfg.secret;
    // UNSAFE: Direct memory access to the secret.
    unsafe {
        let data = (*secret_ptr).clone();
        SENT_DATA = Some(data);
    }
}

// Public API to trigger the transmission process.
// It spawns a thread which calls send_data.
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