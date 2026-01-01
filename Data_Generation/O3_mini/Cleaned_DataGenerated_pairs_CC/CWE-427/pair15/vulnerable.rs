use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
use std::os::raw::c_char;
extern "C" {
    fn getenv(name: *const c_char) -> *const c_char;
}
struct Executor;
impl Executor {
    fn new() -> Self {
        Executor
    }
    fn run_exec(&self) -> String {
        let mut unsafe_path = String::new();
        unsafe {
            let var = std::ffi::CString::new("MALICIOUS_PATH").unwrap();
            let ptr = getenv(var.as_ptr());
            if !ptr.is_null() {
                let c_str = CStr::from_ptr(ptr);
                unsafe_path = c_str.to_string_lossy().into_owned();
            }
        }
        let path_value = if !unsafe_path.is_empty() {
            unsafe_path
        } else {
            env::var("PATH").unwrap_or_else(|_| String::from("/usr/bin"))
        };
        let output = Command::new("echo")
            .arg("vulnerable")
            .env("PATH", path_value)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}
fn main() {
    let executor = Arc::new(Executor::new());
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let executor_clone = Arc::clone(&executor);
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let result = executor_clone.run_exec();
            results_clone.lock().unwrap().push(result);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_result = results.lock().unwrap().get(0).cloned().unwrap_or_default();
    println!("{}", final_result);
}
