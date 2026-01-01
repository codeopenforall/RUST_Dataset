
        // Vulnerable source code
        use std::sync::{Arc, Mutex};
        use std::thread;

        #[derive(Debug)]
        struct Request {
            origin: String,
            host: String,
        }

        struct Gateway {
            trusted: String,
        }

        impl Gateway {
            fn new(trusted: &str) -> Gateway {
                Gateway { trusted: trusted.to_string() }
            }

            unsafe fn validate(&self, req: &Request) -> bool {
                let origin_ptr: *const u8 = req.origin.as_ptr();
                let host_ptr: *const u8 = req.host.as_ptr();
                if origin_ptr == host_ptr {
                    return true;
                }
                if req.host == self.trusted {
                    return true;
                }
                false
            }
        }

        fn main() {
            let gw = Gateway::new("trusted.example.com");
            let req = Request {
                origin: "attacker.example.com".to_string(),
                host: "trusted.example.com".to_string(),
            };

            let shared_gw = Arc::new(gw);
            let shared_req = Arc::new(req);

            let mut handles = vec![];

            for _ in 0..2 {
                let gw_clone = Arc::clone(&shared_gw);
                let req_clone = Arc::clone(&shared_req);
                let handle = thread::spawn(move || {
                    unsafe {
                        gw_clone.validate(&req_clone)
                    }
                });
                handles.push(handle);
            }

            let mut approved = false;
            for handle in handles {
                let res = handle.join().expect("Thread panicked");
                if res {
                    approved = true;
                }
            }

            if approved {
                std::process::exit(1);
            } else {
                std::process::exit(0);
            }
        }
    