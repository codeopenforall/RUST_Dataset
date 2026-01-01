/*
In this corrected version, sensitive data is protected by redacting it in both Debug output and error logs.
The unsafe block is removed from error construction, and the error message is generalized.
This mitigates the CWE-200 risk of exposing sensitive information.
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
        // Sensitive information is redacted.
        write!(f, "Credential {{ secret: [REDACTED] }}")
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
        // Simulate an error without leaking sensitive data.
        Err("Processing error occurred".into())
    }
}

fn main() {
    let cred = Credential { secret: "super_secret_1234".to_string() };
    let proc = Processor::new(cred);
    match proc.process() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => {
            // Log error with redacted message.
            eprintln!("Error occurred: {}", e);
            std::process::exit(1);
        }
    }
}