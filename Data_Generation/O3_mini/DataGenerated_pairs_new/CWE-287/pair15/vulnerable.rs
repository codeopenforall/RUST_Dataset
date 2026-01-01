/////////////////////////////
// Vulnerable Implementation
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

    // Authentication function with improper checks
    fn authenticate(&self, input: &str) -> bool {
        unsafe {
            let secret_bytes = self.secret.as_bytes();
            let input_bytes = input.as_bytes();
            // BUG: Compare only over the minimum length without checking for length equality.
            let len = if secret_bytes.len() < input_bytes.len() { 
                secret_bytes.len() 
            } else { 
                input_bytes.len() 
            };
            for i in 0..len {
                if *input_bytes.get_unchecked(i) != secret_bytes[i] {
                    return false;
                }
            }
            // If the provided token is a prefix of the secret then it erroneously returns true.
            true
        }
    }
}

fn process(input: &str) -> bool {
    let service = Service::new();
    // Spawn a thread to simulate concurrent processing.
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