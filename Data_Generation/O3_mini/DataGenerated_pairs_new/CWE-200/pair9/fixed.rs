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
    // but now avoids leaking sensitive information in the error message.
    pub fn process(secret: &str) -> Result<String, String> {
        unsafe {
            // Although still performing an unsafe operation to simulate low-level processing,
            // we no longer include the raw secret in the error output.
            let _pointer = secret.as_ptr();
            Err("Decryption failed due to an incorrect key".to_owned())
        }
    }

    pub fn run_app() {
        let data = Arc::new(Mutex::new(SecureData::new("SuperSecretKey", "NormalData")));

        let threads: Vec<_> = (0..3)
            .map(|_| {
                let data_clone = data.clone();
                thread::spawn(move || {
                    let secret_value = data_clone.lock().unwrap().secret.clone();
                    let result = process(&secret_value);
                    match result {
                        Ok(res) => println!("Processed result: {}", res),
                        // The panic now only states a generic error message.
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