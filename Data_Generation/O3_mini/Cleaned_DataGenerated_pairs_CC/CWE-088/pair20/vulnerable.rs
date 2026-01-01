use std::process::Command;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::thread;
struct Executor;
impl Executor {
    pub fn run(input: &str) -> Result<String, String> {
        unsafe {
            let raw = CString::new(input).map_err(|e| e.to_string())?;
            let ptr: *const c_char = raw.as_ptr();
            let cmd_string = format!("echo {}", CStr::from_ptr(ptr).to_string_lossy());
            let output = Command::new("sh")
                .arg("-c")
                .arg(cmd_string)
                .output()
                .map_err(|e| e.to_string())?;
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(result.trim().to_string())
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <input>");
        return;
    }
    let user_input = args[1].clone();
    let handle = thread::spawn(move || {
        match Executor::run(&user_input) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
