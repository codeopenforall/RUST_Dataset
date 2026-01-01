use std::sync::Arc;
use std::thread;
struct Configuration {
    key: String,
}
impl Configuration {
    fn new() -> Self {
        Configuration { key: "super_secret".to_string() }
    }
}
fn dispatch(config: Arc<Configuration>) -> String {
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        unsafe {
            let key_ptr = config_clone.key.as_ptr();
            let len = config_clone.key.len();
            let key_slice = std::slice::from_raw_parts(key_ptr, len);
            String::from_utf8_lossy(key_slice).to_string()
        }
    });
    handle.join().unwrap()
}
fn run_app() -> String {
    let config = Arc::new(Configuration::new());
    dispatch(config)
}
fn main() {
    let transmitted = run_app();
    println!("Transmitted data: {}", transmitted);
}
