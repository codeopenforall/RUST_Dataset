use std::env;
use std::thread;
struct Service {
    secret: &'static str,
}
impl Service {
    fn new() -> Self {
        Service { secret: "verysecurekey" }
    }
    fn authenticate(&self, input: &str) -> bool {
        if input.len() != self.secret.len() {
            return false;
        }
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
