use std::sync::{Arc, Mutex};
use std::thread;
struct LogContainer {
    secret: &'static str,
}
impl LogContainer {
    fn new() -> Self {
        LogContainer { secret: "VerySecretInfo" }
    }
    fn process(&self, user_data: &str) -> String {
        format!("User message: {}", user_data)
    }
}
fn simulate(user_data: &str) -> String {
    let container = Arc::new(LogContainer::new());
    let output = Arc::new(Mutex::new(String::new()));
    let container_clone = Arc::clone(&container);
    let output_clone = Arc::clone(&output);
    let handle = thread::spawn(move || {
        let result = container_clone.process(user_data);
        let mut out_lock = output_clone.lock().unwrap();
        *out_lock = result;
    });
    handle.join().unwrap();
    let res = output.lock().unwrap();
    res.clone()
}
fn main() {
    let safe_input = "User provided data";
    let result = simulate(safe_input);
    println!("Result: {}", result);
}
