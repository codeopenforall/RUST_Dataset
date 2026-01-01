use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor {
    data: Arc<String>,
}
impl Executor {
    fn run(&self) -> Result<String, &'static str> {
        let input = self.data.as_str();
        unsafe {
            let cstring = CString::new(input).unwrap();
            let ptr: *const c_char = cstring.as_ptr();
            let arg = CStr::from_ptr(ptr).to_str().unwrap();
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
