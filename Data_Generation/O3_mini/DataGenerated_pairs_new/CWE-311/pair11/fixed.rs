/*
This Rust program simulates configuration loading and secure transmission of a sensitive secret.
Before transmission, the secret is encrypted using a simple XOR-based scheme.
A separate thread is used to simulate the concurrent transmission process.
This prevents the secret from being sent in cleartext, mitigating CWE-311.
*/
const SECRET_VALUE: &str = "myS3cr3t!";
const KEY: u8 = 0xAB; // Simple XOR key for demonstration purposes

struct Config {
    secret: Box<String>,
}

fn load_configuration() -> Config {
    // In practice, configuration might be read from an environment variable or file.
    Config {
        secret: Box::new(String::from(SECRET_VALUE)),
    }
}

fn encrypt_data(secret: &String) -> String {
    // Encrypt the secret using XOR and return a hexadecimal string representation.
    secret
        .bytes()
        .map(|b| format!("{:02x}", b ^ KEY))
        .collect::<String>()
}

pub fn get_transmitted_data() -> String {
    let config = load_configuration();
    // Spawn a thread to mimic concurrent operation.
    let handle = std::thread::spawn(move || encrypt_data(&config.secret));
    handle.join().expect("Thread panicked")
}

fn main() {
    let transmitted = get_transmitted_data();
    println!("Transmitted data: {}", transmitted);
}