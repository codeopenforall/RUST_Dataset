use std::thread;
struct Request {
    origin: String,
}
impl Request {
    fn new(origin: &str) -> Self {
        Request {
            origin: origin.to_string(),
        }
    }
}
unsafe fn quick_check(ptr: *const u8, len: usize) -> bool {
    if len < 8 {
        return false;
    }
    let slice = std::slice::from_raw_parts(ptr, 8);
    slice == b"https://"
}
fn validate(req: &Request) -> bool {
    unsafe { quick_check(req.origin.as_ptr(), req.origin.len()) }
}
fn process(req: Request) {
    let handle = thread::spawn(move || {
        if validate(&req) {
            println!("Request accepted: {}", req.origin);
        } else {
            println!("Request rejected: {}", req.origin);
        }
    });
    handle.join().unwrap();
}
fn main() {
    let req = Request::new("https://trusted.com");
    process(req);
}
