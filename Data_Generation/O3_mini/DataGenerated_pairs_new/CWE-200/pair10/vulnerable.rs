/*
This Rust code simulates a scenario where sensitive data is accidentally leaked in error output.
It uses unsafe pointer conversion and concurrency primitives (Arc and Mutex) to mimic a more complex real‐world scenario.
In this version, the Debug trait and error message construction disclose the secret token.
*/

#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::fmt;
use std::error::Error;

#[derive(Clone)]
struct Credential {
    secret: String,
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Vulnerability: sensitive token is printed directly.
        write!(f, "Credential {{ secret: {} }}", self.secret)
    }
}

struct Processor {
    data: Arc<Mutex<Credential>>,
}

impl Processor {
    fn new(cred: Credential) -> Self {
        Self { data: Arc::new(Mutex::new(cred)) }
    }

    fn process(&self) -> Result<(), Box<dyn Error>> {
        let data = self.data.lock().unwrap();
        if data.secret.is_empty() {
            return Ok(());
        }
        // Simulate a processing error that exposes sensitive data.
        unsafe {
            // Use an unsafe block to simulate complex internal pointer manipulation.
            let ptr = data.secret.as_ptr();
            let len = data.secret.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            let secret_str = std::str::from_utf8(slice).unwrap();
            Err(format!("Processing error with credential: {:?}", secret_str))?
        }
    }
}

fn main() {
    let cred = Credential { secret: "super_secret_1234".to_string() };
    let proc = Processor::new(cred);
    match proc.process() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => {
            // Error log that inadvertently leaks the sensitive token.
            eprintln!("Error occurred: {:?}", e);
            std::process::exit(1);
        }
    }
}