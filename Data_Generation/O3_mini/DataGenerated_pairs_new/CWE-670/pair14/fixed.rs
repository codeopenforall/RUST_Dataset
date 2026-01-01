/*
This revision corrects the control flow in the token verification.
It ensures only the "admin" token is authorized, and the unsafe critical operation
is executed only when proper credentials are provided. The concurrency and unsafe block usage remain.
*/
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    fn verify(&self, token: &str) -> bool {
        // Correct logic: only "admin" is authorized.
        match token {
            "admin" => true,
            _ => false,
        }
    }

    unsafe fn get_value(&self) -> i32 {
        // Simulate an unsafe operation returning a critical constant.
        1337
    }

    fn process(&self, token: &str) -> i32 {
        if self.verify(token) {
            unsafe { self.get_value() }
        } else {
            0
        }
    }
}

fn main() {
    // Shared processor state across threads.
    let proc = Arc::new(Mutex::new(Processor));
    let tokens = vec!["admin", "user"];
    let mut handles = vec![];

    for tok in tokens {
        let proc_clone = Arc::clone(&proc);
        let token = tok.to_owned();
        handles.push(thread::spawn(move || {
            let processor = proc_clone.lock().unwrap();
            processor.process(&token)
        }));
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Result: {}", result);
    }
}