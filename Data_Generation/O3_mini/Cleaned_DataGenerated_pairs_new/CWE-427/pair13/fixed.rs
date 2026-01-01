use std::env;
use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
struct Runner {
    input_search: String,
}
impl Runner {
    fn sanitize(&self) -> String {
        let safe_whitelist = vec!["/usr/bin", "/bin", "/usr/local/bin"];
        let parts: Vec<&str> = self.input_search.split(':').collect();
        let validated: Vec<&str> = parts.into_iter()
            .filter(|d| safe_whitelist.contains(d))
            .collect();
        if validated.is_empty() {
            safe_whitelist.join(":")
        } else {
            validated.join(":")
        }
    }
    fn execute(&self) {
        let validated_search = self.sanitize();
        env::set_var("PATH", &validated_search);
        let raw_ptr = validated_search.as_ptr();
        let len = validated_search.len();
        let shared_ptr = Arc::new(Mutex::new(raw_ptr));
        let handle = {
            let sp = Arc::clone(&shared_ptr);
            thread::spawn(move || {
                unsafe {
                    let stored_ptr = *sp.lock().unwrap();
                    let slice = std::slice::from_raw_parts(stored_ptr, len);
                    let path_str = std::str::from_utf8(slice).unwrap();
                    println!("Validated search path: {}", path_str);
                }
            })
        };
        handle.join().unwrap();
        let output = Command::new("echo")
            .arg("Executing secure command")
            .output()
            .expect("command execution failed");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
fn main() {
    let input_search = env::var("BAD_PATH")
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string());
    let runner = Runner { input_search };
    runner.execute();
}
