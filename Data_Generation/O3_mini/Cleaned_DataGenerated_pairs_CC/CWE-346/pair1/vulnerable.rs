use std::thread;
struct Request {
    origin: String,
    payload: String,
}
struct Service {
    allowed_origin: String,
}
impl Service {
    fn verify_origin(&self, req: &Request) -> bool {
        unsafe {
            let ptr = req.origin.as_ptr();
            if ptr.is_null() {
                false
            } else {
                true
            }
        }
    }
    fn handle(&self, req: Request) -> bool {
        self.verify_origin(&req)
    }
}
fn main() {
    let service = Service {
        allowed_origin: "https://trusted.com".into(),
    };
    let forged_req = Request {
        origin: "https://untrusted.com".into(),
        payload: "sensitive payload".into(),
    };
    let thread_handle = thread::spawn(move || {
        let approval = service.handle(forged_req);
        println!("Request approved: {}", approval);
    });
    thread_handle.join().unwrap();
}
