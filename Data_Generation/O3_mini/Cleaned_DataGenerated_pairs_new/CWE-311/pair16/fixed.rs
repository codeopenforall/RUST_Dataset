use std::env;
use std::thread;
struct Service {
    data: String,
}
impl Service {
    fn new(data: String) -> Self {
        Service { data }
    }
    unsafe fn raw_copy(ptr: *const u8, len: usize) -> Vec<u8> {
        let slice = std::slice::from_raw_parts(ptr, len);
        slice.to_vec()
    }
    fn xor_enc(data: &[u8], key: u8) -> Vec<u8> {
        data.iter().map(|b| b ^ key).collect()
    }
    fn process(&self) -> String {
        let data_bytes = self.data.as_bytes();
        let len = data_bytes.len();
        let copied = unsafe { Service::raw_copy(data_bytes.as_ptr(), len) };
        let encrypted = Service::xor_enc(&copied, 0xAA);
        encrypted.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
fn main() {
    let secret = env::var("SECRET").unwrap_or_else(|_| "sensitive_data".to_string());
    let service = Service::new(secret);
    let handle = thread::spawn(move || {
        service.process()
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}
