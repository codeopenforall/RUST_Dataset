use std::sync::Arc;
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    fn new() -> Self {
        Self {
            secret: "sensitive_data".to_string(),
        }
    }
}
fn transmit(config: &Arc<Config>) -> Vec<u8> {
    let secret_ptr = config.secret.as_ptr();
    let secret_len = config.secret.len();
    let secret_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(secret_ptr, secret_len)
    };
    secret_bytes.to_vec()
}
fn main() {
    let config = Arc::new(Config::new());
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let data = transmit(&config_clone);
        println!("Transmitted data: {:?}", String::from_utf8_lossy(&data));
    });
    handle.join().unwrap();
}
