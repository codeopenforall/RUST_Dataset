use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    fn new(secret: String) -> Self {
        Config { secret }
    }
}
fn process_config(conf: Arc<Mutex<Config>>) {
    let conf_copy = Arc::clone(&conf);
    let handler = thread::spawn(move || {
        let data = conf_copy.lock().unwrap();
        unsafe {
            let ptr = data.secret.as_ptr();
            let len = data.secret.len();
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            let mut file = File::create("output.txt")
                .expect("Unable to create output file");
            file.write_all(raw_slice)
                .expect("Failed to write sensitive data to file");
        }
    });
    handler.join().unwrap();
}
fn main() {
    let config = Arc::new(Mutex::new(Config::new("SensitiveSecret".to_string())));
    process_config(config);
}
