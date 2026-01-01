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
    fn xor_transform(input: &str, key: u8) -> String {
        let transformed: Vec<u8> = input.bytes().map(|b| b ^ key).collect();
        transformed.iter().map(|b| format!("{:02x}", b)).collect()
    }
    fn send(&self) -> String {
        let key: u8 = 0xAA; 
        Self::xor_transform(&self.data, key)
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
