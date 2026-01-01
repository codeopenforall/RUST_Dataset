use std::sync::Arc;
use std::thread;
#[derive(Debug)]
struct Config {
    secret: String,
}
impl Config {
    fn process(&self, flag: bool) -> Result<String, String> {
        if !flag {
            return Err(format!("Error during processing: {:?}", self));
        }
        Ok("Processing complete".to_string())
    }
}
fn main() {
    let config = Config { secret: "SuperSecretPassword".to_string() };
    let shared_config = Arc::new(config);
    let config_clone = Arc::clone(&shared_config);
    let handle = thread::spawn(move || {
        unsafe {
            let raw_ptr: *const Config = Arc::as_ptr(&config_clone);
            let config_ref: &Config = &*raw_ptr;
            let result = config_ref.process(false);
            match result {
                Ok(message) => println!("{}", message),
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
    });
    let _ = handle.join();
}
