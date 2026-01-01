use std::env;
use std::ffi::CStr;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
struct Executor;
impl Executor {
    fn retrieve_path() -> Option<String> {
        if let Ok(val) = env::var("PATH") {
            unsafe {
                let ptr = val.as_ptr() as *const i8;
                let cstr = CStr::from_ptr(ptr);
                return cstr.to_str().ok().map(|s| s.to_string());
            }
        }
        None
    }
    fn launch() -> Result<String, String> {
        if let Some(search_path) = Self::retrieve_path() {
            let output = Command::new("echo")
                .env("PATH", search_path)
                .arg("hello")
                .output();
            match output {
                Ok(result) => {
                    let out_str = String::from_utf8_lossy(&result.stdout).to_string();
                    Ok(out_str)
                },
                Err(e) => Err(format!("Command execution failed: {:?}", e)),
            }
        } else {
            Err("PATH environment variable not set.".to_string())
        }
    }
}
fn main() {
    let shared_flag = Arc::new(Mutex::new(()));
    let flag_clone = Arc::clone(&shared_flag);
    let modifier = thread::spawn(move || {
        let _lock = flag_clone.lock().unwrap();
        for i in 0..5 {
            env::set_var("PATH", format!("/malicious/path{}", i));
            thread::sleep(Duration::from_millis(40));
        }
    });
    modifier.join().unwrap();
    match Executor::launch() {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(err) => println!("Error: {}", err),
    }
}
