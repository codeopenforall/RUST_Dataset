use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Config {
    // Holds a sensitive password in plaintext.
    data: String,
}

impl Config {
    fn new(input: &str) -> Self {
        Self {
            data: input.to_string(),
        }
    }

    // Delivers the sensitive content directly.
    fn send(&self) -> String {
        // Unsafe block mimicking low‐level data accesses that leak content.
        unsafe {
            let raw_ptr = self.data.as_ptr();
            let len = self.data.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            // No encryption, directly converting the bytes to a String.
            String::from_utf8_lossy(bytes).to_string()
        }
    }
}

fn main() {
    // Initialize the configuration with a sensitive secret.
    let conf = Arc::new(Config::new("MySecret"));
    let conf_clone = Arc::clone(&conf);

    let handle = thread::spawn(move || {
        // Introduce a slight delay to mimic a race condition.
        thread::sleep(Duration::from_millis(50));
        // Transmit the secret over an insecure channel.
        let transmitted = conf_clone.send();
        println!("Transmission from thread: {}", transmitted);
        transmitted
    });

    // Concurrent transmission in the main execution path.
    let transmitted_main = conf.send();
    println!("Transmission from main: {}", transmitted_main);

    let transmitted_thread = handle.join().unwrap();
    println!("Final transmitted data: {}", transmitted_thread);
}