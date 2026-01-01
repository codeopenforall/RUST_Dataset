use std::sync::Arc;
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
    fn get_bytes(&self) -> Vec<u8> {
        unsafe {
            let ptr = self.secret.as_ptr();
            let len = self.secret.len();
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
    }
}
pub fn transmit_secret() -> String {
    let config = Config::new("supersecret");
    let arc_config = Arc::new(config);
    let arc_clone = Arc::clone(&arc_config);
    let handle = thread::spawn(move || {
        let bytes = arc_clone.get_bytes();
        String::from_utf8(bytes).unwrap()
    });
    handle.join().unwrap()
}
fn main() {
    let transmitted = transmit_secret();
    println!("Data transmitted: {}", transmitted);
}
