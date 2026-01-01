use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    fn new() -> Self {
        Config { secret: "SuperSecret".to_string() }
    }
}
unsafe fn transmit(raw_ptr: *const u8, len: usize) -> Vec<u8> {
    std::slice::from_raw_parts(raw_ptr, len).to_vec()
}
pub fn get_transmitted() -> Vec<u8> {
    let cfg = Arc::new(Mutex::new(Config::new()));
    let cfg_clone = Arc::clone(&cfg);
    let mut out = Vec::new();
    let handle = thread::spawn(move || {
        let guard = cfg_clone.lock().unwrap();
        let data = guard.secret.as_bytes();
        let transmitted = unsafe {
            transmit(data.as_ptr(), data.len())
        };
        transmitted
    });
    out = handle.join().unwrap();
    out
}
fn run() {
    let transmitted = get_transmitted();
    println!("Transmitted data: {}", String::from_utf8_lossy(&transmitted));
}
fn main() {
    run();
}
