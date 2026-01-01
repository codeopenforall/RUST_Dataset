use std::process::Command;
use std::ffi::{CString, CStr};
use std::thread;

struct Worker;

impl Worker {
    // This helper function returns the file name of the executable.
    fn get_target(&self) -> &'static str {
        // Vulnerability: using a relative path allows an attacker to hijack the executable search path.
        "helper_program"
    }

    fn run(&self) -> Result<String, String> {
        let target = self.get_target();
        // Create a C-compatible string from the target.
        let c_target = CString::new(target).unwrap();
        unsafe {
            // Unsafe pointer manipulation: obtaining a raw pointer without additional checks.
            let ptr = c_target.as_ptr();
            let cmd = CStr::from_ptr(ptr).to_str().unwrap();
            // CWE-426: Execute the command using a relative (untrusted) search path.
            let output = Command::new(cmd)
                .output()
                .map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        }
    }
}

fn main() {
    let worker = Worker;
    // Spawn a thread to simulate concurrent execution.
    let handle = thread::spawn(move || {
        match worker.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}