use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
pub struct Executor {
    config: Arc<String>,
    counter: Arc<Mutex<u32>>,
}
impl Executor {
    pub fn new(cfg: String) -> Self {
        Executor {
            config: Arc::new(cfg),
            counter: Arc::new(Mutex::new(0)),
        }
    }
    pub fn execute(&self, user_input: &str) -> Result<String, String> {
        let config_str = unsafe {
            let ptr = self.config.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, self.config.len());
            std::str::from_utf8(slice).unwrap_or("")
        };
        let counter_clone = self.counter.clone();
        let config_value = config_str.to_string();
        let input_value = user_input.to_string();
        let handle = thread::spawn(move || {
            Command::new("echo")
                .arg(config_value)
                .arg(input_value)
                .output()
        });
        if let Ok(mut cnt) = self.counter.lock() {
            *cnt += 1;
        }
        match handle.join() {
            Ok(Ok(output)) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
            _ => Err("Execution failure".to_string()),
        }
    }
}
fn main() {
    let executor = Executor::new("PREFIX".to_string());
    let res = executor.execute("hello; echo INJECTED");
    println!("Result: {:?}", res);
}
