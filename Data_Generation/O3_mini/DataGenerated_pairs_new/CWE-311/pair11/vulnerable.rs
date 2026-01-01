/*
This Rust program simulates a configuration loader that obtains a sensitive secret
and transmits it over an insecure channel. It uses unsafe blocks to operate on raw pointers,
and spawns a thread to simulate asynchronous transmission. The secret is transmitted in cleartext,
exposing it to interception (CWE-311).
*/
const SECRET_VALUE: &str = "myS3cr3t!";

struct Config {
    secret: Box<String>,
}

fn load_configuration() -> Config {
    // In practice, configuration might be read from an environment variable or file.
    Config {
        secret: Box::new(String::from(SECRET_VALUE)),
    }
}

fn transmit_data(secret: &String) -> String {
    let mut result = String::new();
    unsafe {
        // UNSAFE: Convert the secret into a raw pointer and build a slice from it.
        // This bypasses Rust's safety checks and uses the secret in cleartext.
        let raw = secret.as_ptr();
        let slice = std::slice::from_raw_parts(raw, secret.len());
        result = String::from_utf8_lossy(slice).to_string();
    }
    // Insecure transmission: the cleartext secret is returned as the transmitted data.
    result
}

pub fn get_transmitted_data() -> String {
    let config = load_configuration();
    // Spawn a thread to mimic concurrent operation.
    let handle = std::thread::spawn(move || transmit_data(&config.secret));
    handle.join().expect("Thread panicked")
}

fn main() {
    let transmitted = get_transmitted_data();
    println!("Transmitted data: {}", transmitted);
}