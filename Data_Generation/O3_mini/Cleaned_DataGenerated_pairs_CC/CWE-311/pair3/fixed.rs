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
fn secure_transform(data: &str) -> String {
    let xor_key: u8 = 0xAA;
    data.bytes()
        .map(|b| format!("{:02X}", b ^ xor_key))
        .collect()
}
fn dispatch(config: Arc<Configuration>) -> String {
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        secure_transform(&config_clone.key)
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
