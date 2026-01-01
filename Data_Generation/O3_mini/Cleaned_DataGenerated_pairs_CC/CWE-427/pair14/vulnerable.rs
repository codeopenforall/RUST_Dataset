use std::env;
use std::ffi::CString;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Runner {
    prog: String,
    params: Vec<String>,
}
impl Runner {
    fn new(prog: &str, params: Vec<String>) -> Self {
        Runner { 
            prog: prog.to_string(), 
            params 
        }
    }
    unsafe fn execute(&self) -> Result<String, String> {
        if let Ok(user_path) = env::var("MALICIOUS_PATH") {
            let c_user_path = CString::new(user_path.clone()).map_err(|e| e.to_string())?;
            let ptr = c_user_path.as_ptr();
            let slice = std::slice::from_raw_parts(ptr as *const u8, user_path.len());
            let new_path = String::from_utf8_lossy(slice).to_string();
            env::set_var("PATH", new_path);
        }
        let output = Command::new(&self.prog)
            .args(&self.params)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!(
                "Process error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}
fn main() {
    let shared_runner = Arc::new(Mutex::new(Runner::new("echo", vec!["Hello".to_string()])));
    let mut threads = vec![];
    for _ in 0..4 {
        let runner_copy = Arc::clone(&shared_runner);
        let handle = thread::spawn(move || {
            let guard = runner_copy.lock().unwrap();
            unsafe {
                match guard.execute() {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}
