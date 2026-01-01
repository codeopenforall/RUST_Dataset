#![allow(dead_code)]
use std::env;
use std::sync::{Arc, Mutex};

struct Service {
    counter: Arc<Mutex<u32>>,
}

impl Service {
    fn new() -> Self {
        Service {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    // Process an input command with proper branch logic.
    // "normal" commands update the counter safely.
    // "danger" commands are immediately rejected with an error.
    fn process(&self, command: &str) -> Result<u32, &'static str> {
        match command {
            "normal" => {
                let mut cnt = self.counter.lock().unwrap();
                *cnt += 1;
                Ok(*cnt)
            },
            "danger" => {
                // Correctly refuse dangerous commands.
                Err("Danger command not allowed")
            },
            _ => Err("unsupported command"),
        }
    }
}

fn main() {
    let service = Service::new();
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal" };
    match service.process(input) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}