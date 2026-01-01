use std::env;
use std::sync::Arc;
use std::thread;
struct Gatekeeper {
    secret: String,
}
impl Gatekeeper {
    fn new() -> Self {
        Gatekeeper { secret: "password".to_string() }
    }
    fn verify(&self, token: &str) -> bool {
        self.secret.as_bytes() == token.as_bytes()
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <token>", args[0]);
        std::process::exit(1);
    }
    let input_token = args[1].clone();
    let gate = Arc::new(Gatekeeper::new());
    let mut handles = vec![];
    for _ in 0..5 {
        let gate_clone = Arc::clone(&gate);
        let token_clone = input_token.clone();
        let handle = thread::spawn(move || {
            if gate_clone.verify(&token_clone) {
                println!("Access Granted");
            } else {
                println!("Access Denied");
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}
