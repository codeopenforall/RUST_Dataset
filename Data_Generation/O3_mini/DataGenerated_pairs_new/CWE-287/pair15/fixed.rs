/////////////////////////////
// Corrected Implementation
/////////////////////////////
use std::env;
use std::thread;

struct Service {
    secret: &'static str,
}

impl Service {
    fn new() -> Self {
        Service { secret: "verysecurekey" }
    }

    // Secure authentication function that properly checks length and content.
    fn authenticate(&self, input: &str) -> bool {
        // First verify the length exactly matches.
        if input.len() != self.secret.len() {
            return false;
        }
        // Perform a safe, end-to-end comparison.
        self.secret.as_bytes() == input.as_bytes()
    }
}

fn process(input: &str) -> bool {
    let service = Service::new();
    let auth_clone = input.to_string();
    let handle = thread::spawn(move || {
        service.authenticate(&auth_clone)
    });
    handle.join().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <token>", args[0]);
        std::process::exit(1);
    }
    let token = &args[1];
    if process(token) {
        println!("Access Granted");
    } else {
        println!("Access Denied");
    }
}