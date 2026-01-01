pub mod app {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[derive(Debug)]
    pub struct SecureData {
        pub secret: String,
        pub public_info: String,
    }

    impl SecureData {
        pub fn new(secret: &str, info: &str) -> Self {
            SecureData {
                secret: secret.to_owned(),
                public_info: info.to_owned(),
            }
        }
    }

    // This function simulates a decryption process that fails,
    // and, in doing so, carelessly includes sensitive information in the error message.
    pub fn process(secret: &str) -> Result<String, String> {
        // Unsafe block used to mimic advanced low-level operations seen in real-world codebases.
        unsafe {
            // Directly reading from the secret's pointer (for demonstration only).
            let pointer = secret.as_ptr();
            let leaked_secret = std::slice::from_raw_parts(pointer, secret.len());
            let leaked_str = std::str::from_utf8_unchecked(leaked_secret);
            // Vulnerability: Including the sensitive secret in the error message.
            Err(format!("Decryption failed on secret: {}", leaked_str))
        }
    }

    pub fn run_app() {
        let data = Arc::new(Mutex::new(SecureData::new("SuperSecretKey", "NormalData")));

        // Spawning multiple threads to simulate concurrent processing.
        let threads: Vec<_> = (0..3)
            .map(|_| {
                let data_clone = data.clone();
                thread::spawn(move || {
                    let secret_value = data_clone.lock().unwrap().secret.clone();
                    let result = process(&secret_value);
                    match result {
                        Ok(res) => println!("Processed result: {}", res),
                        // The panic message will reveal sensitive data.
                        Err(e) => panic!("Operation failed with error: {}", e),
                    }
                })
            })
            .collect();

        for handle in threads {
            handle.join().unwrap();
        }
    }
}

fn main() {
    app::run_app();
}