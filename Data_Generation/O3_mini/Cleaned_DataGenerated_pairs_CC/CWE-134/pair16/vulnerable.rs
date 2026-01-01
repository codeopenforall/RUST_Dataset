use std::sync::{Arc, Mutex};
use std::thread;
struct LogHolder {
    secret: &'static str,
}
impl LogHolder {
    fn new() -> Self {
        LogHolder { secret: "VerySecretInfo" }
    }
    fn process(&self, user_fmt: &str) -> String {
        let secret_ptr = self.secret.as_ptr();
        let secret_str: &str = unsafe {
            let slice = std::slice::from_raw_parts(secret_ptr, self.secret.len());
            std::str::from_utf8_unchecked(slice)
        };
        format!(user_fmt, secret_str)
    }
}
fn simulate(user_input: &str) -> String {
    let logger = Arc::new(LogHolder::new());
    let output = Arc::new(Mutex::new(String::new()));
    let logger_clone = Arc::clone(&logger);
    let output_clone = Arc::clone(&output);
    let handle = thread::spawn(move || {
        let result = logger_clone.process(user_input);
        let mut out_lock = output_clone.lock().unwrap();
        *out_lock = result;
    });
    handle.join().unwrap();
    let res = output.lock().unwrap();
    res.clone()
}
fn main() {
    let user_controlled_input = "Alert: {}";
    let result = simulate(user_controlled_input);
    println!("Result: {}", result);
}
