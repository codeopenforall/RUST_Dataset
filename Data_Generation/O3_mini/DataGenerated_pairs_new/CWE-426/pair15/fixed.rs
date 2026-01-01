//////////////////////////////////////////////
// Fixed version mitigating CWE-426 issue
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;

fn run_exec() -> Result<String, String> {
    let result = Arc::new(Mutex::new(String::new()));
    // Fix: using an absolute path to ensure the correct executable/library is loaded.
    let cmd = "/usr/bin/danger_exec"; // absolute path enforced
    let result_clone = Arc::clone(&result);
    let handle = thread::spawn(move || {
        unsafe {
            // Unsafe FFI-like usage retained but now used with a secure absolute path.
            let c_cmd = CString::new(cmd).unwrap();
            let ptr = c_cmd.as_ptr();
            let s = std::ffi::CStr::from_ptr(ptr).to_str().unwrap();
            let mut res = result_clone.lock().unwrap();
            *res = s.to_owned();
        }
    });
    handle.join().unwrap();

    let final_command = result.lock().unwrap();
    // Verify that the command is absolute.
    if !final_command.starts_with('/') {
        Err("Untrusted search path vulnerability triggered".to_string())
    } else {
        Ok("Command executed securely".to_string())
    }
}

fn main() {
    match run_exec() {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }
}