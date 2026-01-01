////////////////////////////////////////////////////////////
// Vulnerable implementation susceptible to CWE-288.
// This service exposes a reserved interface intended for debug
// use, but it inadvertently bypasses proper authentication.
// Unsafe blocks and concurrency are used to mimic real-world usage.
////////////////////////////////////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

pub struct Service;

impl Service {
    pub fn new() -> Self {
        Service {}
    }

    // Regular request handling: returns true only for correct token.
    pub unsafe fn process(&self, token: Option<&str>) -> bool {
        token == Some("secret")
    }

    // Reserved (debug) interface intended for internal diagnostics.
    // Vulnerability: This function bypasses authentication by always
    // returning true, ignoring the provided token.
    pub unsafe fn reserved_query(&self, _token: Option<&str>) -> bool {
        // Insecure alternate path: bypasses auth without validation.
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let service = Arc::new(Service::new());

    // Simulate a concurrent normal request thread.
    let svc_clone = Arc::clone(&service);
    let normal_handle = thread::spawn(move || {
        // This is a normal endpoint requiring proper credentials.
        unsafe { svc_clone.process(Some("secret")) }
    });

    // Meanwhile, the reserved debug endpoint is exposed and callable.
    // It bypasses the authentication check.
    let debug_result = unsafe { service.reserved_query(None) };

    // Wait for the normal request thread.
    let normal_result = normal_handle.join().unwrap();

    println!("Normal endpoint returned: {}", normal_result);
    println!("Reserved endpoint returned: {}", debug_result);
}