use std::sync::Arc;
use std::thread;
struct Config {
    threshold: u32,
    message: String,
}
impl Config {
    fn validate(&self) -> bool {
        self.threshold > 50 && self.message == "default"
    }
}
fn load_configuration() -> Config {
    Config {
        threshold: 100,
        message: "default".to_string(),
    }
}
fn main() {
    let config_shared = Arc::new(load_configuration());
    let config_for_thread = Arc::clone(&config_shared);
    let handler = thread::spawn(move || {
        if config_for_thread.validate() {
            println!("Configuration validated successfully.");
        } else {
            println!("Configuration validation failed.");
        }
    });
    handler.join().unwrap();
    println!("Main thread execution complete.");
}
