use std::thread;

struct Request {
    origin: String,
    payload: String,
}

struct Service {
    allowed_origin: String,
}

impl Service {
    // This verification routine employs an unsafe block that mistakenly trusts
    // the header simply based on the pointer's non-nullity rather than its actual content.
    fn verify_origin(&self, req: &Request) -> bool {
        unsafe {
            // Dereference the pointer to the origin string.
            let ptr = req.origin.as_ptr();
            // If the pointer is not null, the request is approved without proper validation.
            if ptr.is_null() {
                false
            } else {
                // Vulnerability: the actual string contents are not compared.
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