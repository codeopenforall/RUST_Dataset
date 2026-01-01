///////////////////////////////////////////////////////////////
// Corrected implementation with simple encryption of secret data
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

    // Unsafe helper remains unchanged as lower-level memory copy
    unsafe fn raw_copy(ptr: *const u8, len: usize) -> Vec<u8> {
        let slice = std::slice::from_raw_parts(ptr, len);
        slice.to_vec()
    }

    // Simple XOR encryption helper for demonstration purposes.
    // (Note: In production, use a vetted cryptographic library.)
    fn xor_enc(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|b| b ^ key).collect()
    }

    // The processing method now encrypts the sensitive data before output.
    fn process(&self) -> String {
        let data_bytes = self.data.as_bytes();
        let len = data_bytes.len();
        // Perform the unsafe copy as before
        let copied = unsafe { Service::raw_copy(data_bytes.as_ptr(), len) };
        // Encrypt the copied data using a simple XOR algorithm
        let encrypted = Service::xor_enc(&copied, 0xAA);
        // Return the encrypted data as a hexadecimal string
        encrypted.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

fn main() {
    // Read the secret; in production this should come from a secured source.
    let secret = env::var("SECRET").unwrap_or_else(|_| "sensitive_data".to_string());
    let service = Service::new(secret);
    let handle = thread::spawn(move || {
        service.process()
    });

    let result = handle.join().unwrap();
    // Encrypted secret is shown, mitigating plain text exposure
    println!("Result: {}", result);
}