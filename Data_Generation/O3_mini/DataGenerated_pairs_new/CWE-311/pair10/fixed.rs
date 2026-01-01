use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Config {
    // Holds a sensitive password.
    data: String,
}

impl Config {
    fn new(input: &str) -> Self {
        Self {
            data: input.to_string(),
        }
    }

    // A simple XOR-based encryption routine; in real applications, use a robust cryptographic library.
    fn xor_transform(input: &str, key: u8) -> String {
        let transformed: Vec<u8> = input.bytes().map(|b| b ^ key).collect();
        // Represent the encrypted data as a hex string.
        transformed.iter().map(|b| format!("{:02x}", b)).collect()
    }

    // Transmits the data after applying encryption.
    fn send(&self) -> String {
        let key: u8 = 0xAA; // Example fixed key for demonstration.
        Self::xor_transform(&self.data, key)
    }
}

fn main() {
    // Initialize the configuration with a sensitive secret.
    let conf = Arc::new(Config::new("MySecret"));
    let conf_clone = Arc::clone(&conf);

    let handle = thread::spawn(move || {
        // Simulate concurrent processing with delay.
        thread::sleep(Duration::from_millis(50));
        // Transmit the secret after encryption.
        let transmitted = conf_clone.send();
        println!("Transmission from thread: {}", transmitted);
        transmitted
    });

    // Concurrent transmission.
    let transmitted_main = conf.send();
    println!("Transmission from main: {}", transmitted_main);

    let transmitted_thread = handle.join().unwrap();
    println!("Final transmitted data: {}", transmitted_thread);
}