////////////////////////////////////////////////////////////
// Fixed implementation: Proper authentication is enforced
// even on the previously reserved (debug) interface.
// The reserved interface now verifies the provided credentials.
////////////////////////////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

pub struct Service;

impl Service {
    pub fn new() -> Self {
        Service {}
    }

    // Regular request handling remains unchanged.
    pub unsafe fn process(&self, token: Option<&str>) -> bool {
        token == Some("secret")
    }

    // Reserved (debug) interface now properly requires a valid token.
    pub unsafe fn reserved_query(&self, token: Option<&str>) -> bool {
        // Instead of bypassing authentication, it verifies the token.
        token == Some("secret")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let service = Arc::new(Service::new());

    // Simulate concurrent normal request.
    let svc_clone = Arc::clone(&service);
    let normal_handle = thread::spawn(move || {
        unsafe { svc_clone.process(Some("secret")) }
    });

    // Protected reserved route: requires proper token.
    let reserved_result = unsafe { service.reserved_query(None) };

    let normal_result = normal_handle.join().unwrap();

    println!("Normal endpoint returned: {}", normal_result);
    println!("Reserved endpoint returned: {}", reserved_result);
}