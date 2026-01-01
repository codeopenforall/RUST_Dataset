use std::env;
use std::thread;
#[derive(Clone)]
struct HttpRequest {
    pub origin: String,
    pub host: String,
}
static mut ALLOWED_PTR: *const u8 = 0 as *const u8;
static mut ALLOWED_LEN: usize = 0;
fn setup(req: &HttpRequest) {
    unsafe {
        ALLOWED_PTR = req.origin.as_ptr();
        ALLOWED_LEN = req.origin.len();
    }
}
fn check(req: &HttpRequest) -> bool {
    unsafe {
        let slice = std::slice::from_raw_parts(ALLOWED_PTR, ALLOWED_LEN);
        let stored = std::str::from_utf8_unchecked(slice);
        stored == req.origin
    }
}
fn process(req: HttpRequest) -> bool {
    setup(&req);
    let handle = thread::spawn(move || {
        check(&req)
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
