use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Config {
    data: String,
}
impl Config {
    fn new(input: &str) -> Self {
        Self {
            data: input.to_string(),
        }
    }
    fn send(&self) -> String {
        unsafe {
            let raw_ptr = self.data.as_ptr();
            let len = self.data.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).to_string()
        }
    }
}
fn main() {
    let conf = Arc::new(Config::new("MySecret"));
    let conf_clone = Arc::clone(&conf);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let transmitted = conf_clone.send();
        println!("Transmission from thread: {}", transmitted);
        transmitted
    });
    let transmitted_main = conf.send();
    println!("Transmission from main: {}", transmitted_main);
    let transmitted_thread = handle.join().unwrap();
    println!("Final transmitted data: {}", transmitted_thread);
}
