use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};
fn launch() -> Result<String, String> {
    unsafe {
        let raw_ptr = CString::new("ls").expect("CString failed").into_raw();
        let cmd = CStr::from_ptr(raw_ptr)
            .to_str()
            .map_err(|e| format!("Conversion error: {}", e))?;
        let output = Command::new(cmd)
            .output()
            .map_err(|e| format!("Execution error: {}", e))?;
        let _ = CString::from_raw(raw_ptr);
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
fn process() -> Result<(), String> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let res = launch();
            let mut vec = results_clone.lock().unwrap();
            vec.push(res);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().map_err(|_| "Thread join error".to_string())?;
    }
    let vec = results.lock().unwrap();
    if vec.iter().any(|r| r.is_err()) {
        Err("One or more commands failed".into())
    } else {
        Ok(())
    }
}
fn main() {
    match process() {
        Ok(_) => println!("Completed successfully"),
        Err(e) => println!("Error: {}", e)
    }
}
