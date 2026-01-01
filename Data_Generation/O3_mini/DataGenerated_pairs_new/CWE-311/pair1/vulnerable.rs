//////////////////////////////////////////////////////////
// Vulnerable code example for missing encryption of sensitive data.
// The program stores a sensitive secret (e.g. configuration value)
// directly in plaintext on disk. It also uses unsafe blocks to obtain
// raw pointers to the secret data and spawns a thread to simulate concurrent
// usage. This mimics real-world oversight where sensitive data is written unencrypted.
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
    // Unsafe method providing raw pointer access to the secret bytes.
    unsafe fn ptr_reveal(&self) -> *const u8 {
        self.secret.as_ptr()
    }
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
    // Create a configuration instance with sensitive data stored in plaintext.
    let config = Config {
        secret: "my_super_secret".to_owned(),
    };
    let arc_conf = Arc::new(Mutex::new(config));

    // Access sensitive secret using an unsafe raw pointer.
    let plain_secret = {
        let conf_lock = arc_conf.lock().unwrap();
        let ptr = unsafe { conf_lock.ptr_reveal() };
        let len = conf_lock.secret.len();
        unsafe {
            String::from_utf8_lossy(std::slice::from_raw_parts(ptr, len)).into_owned()
        }
    };

    // Vulnerability: Writing sensitive secret directly to disk in clear text.
    let mut file = File::create("secret.txt").expect("cannot create file");
    file.write_all(plain_secret.as_bytes())
        .expect("cannot write data");

    process(arc_conf);
}