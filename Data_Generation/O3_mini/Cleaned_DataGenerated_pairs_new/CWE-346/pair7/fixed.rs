use std::env;
use std::thread;
#[derive(Clone)]
struct HttpRequest {
    pub origin: String,
    pub host: String,
}
static TRUSTED_ORIGIN: &str = "https://trusteddomain.com";
fn validate(req: &HttpRequest) -> bool {
    req.origin == TRUSTED_ORIGIN
}
fn process(req: HttpRequest) -> bool {
    let handle = thread::spawn(move || {
        validate(&req)
    });
    handle.join().unwrap()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <origin> <host>", args[0]);
        std::process::exit(1);
    }
    let req = HttpRequest {
        origin: args[1].clone(),
        host: args[2].clone(),
    };
    if process(req) {
        println!("Access granted");
        std::process::exit(0);
    } else {
        println!("Access denied");
        std::process::exit(1);
    }
}
