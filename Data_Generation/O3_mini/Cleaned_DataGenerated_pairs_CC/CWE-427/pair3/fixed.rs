#![allow(unused)]
use std::env;
use std::thread;
struct Runner;
impl Runner {
    fn execute(&self) -> Result<String, String> {
        let default_safe_path = "/usr/bin:/bin".to_string();
        let user_input = env::var("SEARCH_PATH").unwrap_or_else(|_| default_safe_path.clone());
        let sanitized: Vec<&str> = user_input.split(':').filter(|segment| segment.starts_with('/')).collect();
        let safe_value = if sanitized.is_empty() {
            default_safe_path
        } else {
            sanitized.join(":")
        };
        let safe_clone = safe_value.clone();
        let handle = thread::spawn(move || {
            env::set_var("PATH", safe_clone);
        });
        handle.join().unwrap();
        let current_path = env::var("PATH").unwrap_or_default();
        Ok(current_path)
    }
}
fn main() {
    let runner = Runner;
    match runner.execute() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}
