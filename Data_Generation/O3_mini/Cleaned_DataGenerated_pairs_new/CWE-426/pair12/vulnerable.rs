use std::process::Command;
use std::ffi::{CString, CStr};
use std::thread;
struct Worker;
impl Worker {
    fn get_target(&self) -> &'static str {
        "helper_program"
    }
    fn run(&self) -> Result<String, String> {
        let target = self.get_target();
        let c_target = CString::new(target).unwrap();
        unsafe {
            let ptr = c_target.as_ptr();
            let cmd = CStr::from_ptr(ptr).to_str().unwrap();
            let output = Command::new(cmd)
                .output()
                .map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        }
    }
}
fn main() {
    let worker = Worker;
    let handle = thread::spawn(move || {
        match worker.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
