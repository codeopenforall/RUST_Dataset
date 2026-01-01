use std::thread;

struct Request {
    origin: String,
    payload: String,
}

struct Service {
    allowed_origin: String,
}

impl Service {
    // This correct verification function securely compares the origin header content
    // against the known, trusted value.
    fn verify_origin(&self, req: &Request) -> bool {
        req.origin == self.allowed_origin
    }

    fn handle(&self, req: Request) -> bool {
        self.verify_origin(&req)
    }
}

fn main() {
    let service = Service {
        allowed_origin: "https://trusted.com".into(),
    };

    // Simulate a forged request coming from an untrusted origin.
    let forged_req = Request {
        origin: "https://untrusted.com".into(),
        payload: "sensitive payload".into(),
    };

    // Spawn a thread to mimic concurrent request handling.
    let thread_handle = thread::spawn(move || {
        let approval = service.handle(forged_req);
        println!("Request approved: {}", approval);
    });
    thread_handle.join().unwrap();
}