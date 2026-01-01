use std::thread;
pub struct Config {
    pub secret: String,
}
impl Config {
    pub fn new() -> Self {
        Self {
            secret: "supersecret".to_string(),
        }
    }
}
fn transmit(cfg: &Config) -> String {
    let data: String;
    unsafe {
        let ptr = cfg.secret.as_ptr();
        let len = cfg.secret.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        data = String::from_utf8_unchecked(slice.to_vec());
    }
    data  
}
pub fn process_output() -> String {
    let cfg = Config::new();
    let handle = thread::spawn(move || {
        transmit(&cfg)
    });
    handle.join().unwrap()
}
fn main() {
    let output = process_output();
    println!("Transmitted data: {}", output);
}
