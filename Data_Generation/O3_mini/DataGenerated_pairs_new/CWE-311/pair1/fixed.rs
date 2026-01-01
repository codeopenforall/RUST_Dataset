//////////////////////////////////////////////////////////
// Fixed code example for missing encryption of sensitive data.
// This version encrypts the sensitive secret before writing it to disk.
// A simple XOR-based encryption (demo purpose) is applied with a fixed key.
// The unsafe handling and concurrent access structure remain similar, but
// the stored data is no longer in clear text.
// Note: Do not include test or oracle code in this snippet.
//////////////////////////////////////////////////////////

use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

struct Config {
    secret: String,
}

impl Config {
    // Still provides unsafe raw pointer access for demonstration.
    unsafe fn ptr_reveal(&self) -> *const u8 {
        self.secret.as_ptr()
    }
}

// A simple XOR "encryption" function for demonstration purposes.
fn encrypt(data: &str, key: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ key).collect()
}

fn decrypt(data: &[u8], key: u8) -> String {
    let decrypted: Vec<u8> = data.iter().map(|b| b ^ key).collect();
    String::from_utf8_lossy(&decrypted).into_owned()
}

fn process(conf: Arc<Mutex<Config>>) {
    // Spawn a thread to simulate concurrent access and unsafe usage.
    let conf_clone = Arc::clone(&conf);
    thread::spawn(move || {
        let conf_lock = conf_clone.lock().unwrap();
        // Using unsafe block to access the raw pointer.
        let secret_ptr = unsafe { conf_lock.ptr_reveal() };
        let secret_len = conf_lock.secret.len();
        let secret_slice = unsafe { std::slice::from_raw_parts(secret_ptr, secret_len) };
        // For demonstration, print the secret in hexadecimal representation.
        for byte in secret_slice {
            print!("{:02x}", byte);
        }
        println!();
    })
    .join()
    .unwrap();
}

fn main() {
    // Create a configuration instance with sensitive data.
    let config = Config {
        secret: "my_super_secret".to_owned(),
    };
    let arc_conf = Arc::new(Mutex::new(config));

    // Define an encryption key (demo purposes only; in production use a secure algorithm).
    let key: u8 = 0xAA;

    // Access the secret using an unsafe raw pointer.
    let plain_secret = {
        let conf_lock = arc_conf.lock().unwrap();
        let ptr = unsafe { conf_lock.ptr_reveal() };
        let len = conf_lock.secret.len();
        unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).into_owned()
        }
    };

    // Encrypt the secret before writing to disk.
    let encrypted_data = encrypt(&plain_secret, key);

    let mut file = File::create("secret.txt").expect("cannot create file");
    file.write_all(&encrypted_data)
        .expect("cannot write data");

    process(arc_conf);

    // Verification: decrypt and assert correctness.
    let decrypted = decrypt(&encrypted_data, key);
    assert_eq!(decrypted, plain_secret, "Decryption failed");
}