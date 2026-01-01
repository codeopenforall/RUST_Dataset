use std::fmt;
use std::sync::{Arc, mpsc};
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    fn new(secret: &str) -> Config {
        Config {
            secret: secret.to_string(),
        }
    }
}
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config {{ secret: {} }}", self.secret)
    }
}
pub fn get_debug() -> String {
    let config = Config::new("super-secret-1234");
    format!("{:?}", config)
}
fn run() -> Result<(), &'static str> {
    let config = Arc::new(Config::new("super-secret-1234"));
    let (tx, rx) = mpsc::channel();
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        unsafe {
            let ptr = config_clone.secret.as_ptr();
            let secret_slice = std::slice::from_raw_parts(ptr, config_clone.secret.len());
            if secret_slice[0] == b's' {
                // FLAW: printing the debug representation inadvertently discloses sensitive data.
                eprintln!("Error: encountered configuration error in {:?}", config_clone);
                tx.send(Err("configuration error")).unwrap();
                return;
            }
            tx.send(Ok(())).unwrap();
        }
    });
    let result = rx.recv().unwrap();
    handle.join().unwrap();
    result
}
fn main() {
    let _ = run();
}
