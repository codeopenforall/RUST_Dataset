use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Executor {
    data: Arc<String>,
}

impl Executor {
    // This method builds a shell command using untrusted input without proper sanitization.
    // It uses an unsafe block to convert a pointer to a C string, then passes the derived string to a shell.
    fn run(&self) -> Result<String, &'static str> {
        let input = self.data.as_str();
        unsafe {
            // Create a CString from untrusted input.
            let cstring = CString::new(input).unwrap();
            let ptr: *const c_char = cstring.as_ptr();
            // Reconstruct the string from a raw pointer (unsafe conversion).
            let arg = CStr::from_ptr(ptr).to_str().unwrap();
            // Here, the input is directly interpolated into a shell command.
            // If the input contains shell metacharacters, an attacker may inject arbitrary commands.
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("echo {}", arg))
                .output()
                .map_err(|_| "failed to execute command")?;
            if !output.status.success() {
                return Err("command did not return success");
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
    }
}

fn main() {
    // In a real-world scenario, this input might come from an untrusted source.
    // Here, it contains shell metacharacters to simulate an injection attack.
    let input = "hello; echo injected";
    let exec = Executor {
        data: Arc::new(input.to_string()),
    };

    let handle = thread::spawn(move || {
        match exec.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

    handle.join().unwrap();
}