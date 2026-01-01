///////////////////////////////////////////////////////////////
// Vulnerable implementation using plain-text handling of secret
///////////////////////////////////////////////////////////////
use std::env;
use std::thread;

struct Service {
    data: String,
}

impl Service {
    fn new(data: String) -> Self {
        Service { data }
    }

    // Unsafe helper to copy a memory region pointed by raw pointer
    unsafe fn raw_copy(ptr: *const u8, len: usize) -> Vec<u8> {
        let slice = std::slice::from_raw_parts(ptr, len);
        slice.to_vec()
    }

    // This method retrieves the sensitive data without any encryption.
    fn process(&self) -> String {
        let data_bytes = self.data.as_bytes();
        let len = data_bytes.len();
        // Unsafe copy of the secret memory (uses raw pointer arithmetic)
        let copied = unsafe { Service::raw_copy(data_bytes.as_ptr(), len) };
        // FORGOT TO ENCRYPT: returns the sensitive data in clear-text
        String::from_utf8_lossy(&copied).into_owned()
    }
}

fn main() {
    // In a realistic scenario, the secret would come from a secure environment,
    // but here it defaults to a hard-coded sensitive string.
    let secret = env::var("SECRET").unwrap_or_else(|_| "sensitive_data".to_string());
    let service = Service::new(secret);
    let handle = thread::spawn(move || {
        service.process()
    });

    let result = handle.join().unwrap();
    // The secret is output directly in plain text
    println!("Result: {}", result);
}