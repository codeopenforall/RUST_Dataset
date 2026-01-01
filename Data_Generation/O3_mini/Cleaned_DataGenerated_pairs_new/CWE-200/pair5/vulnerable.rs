use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;
struct SecureData {
    secret: String,
}
impl fmt::Debug for SecureData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SecureData {{ secret: {} }}", self.secret)
    }
}
impl SecureData {
    fn new(secret: String) -> Self {
        SecureData { secret }
    }
    fn process(&self, value: i32) -> Result<i32, String> {
        if value < 0 || value > 100 {
            let exposed = unsafe {
                let ptr: *const String = &self.secret;
                (*ptr).clone()
            };
            return Err(format!(
                "Invalid value: {}. Debug info: {:?}",
                value, exposed
            ));
        }
        Ok(value * 2)
    }
}
fn main() {
    let data = Arc::new(Mutex::new(SecureData::new("SuperSecret123".to_string())));
    let mut threads = vec![];
    for _ in 0..4 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let guard = data_clone.lock().unwrap();
            match guard.process(101) { 
                Ok(result) => println!("Computation succeeded: {}", result),
                Err(err) => panic!("Computation failed: {}", err),
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        let _ = handle.join();
    }
}
