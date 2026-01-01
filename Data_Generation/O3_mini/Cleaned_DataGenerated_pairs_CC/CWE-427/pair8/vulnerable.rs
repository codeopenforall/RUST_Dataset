------------------ vulnerable.rs ------------------
#![allow(unused_unsafe)]
use std::env;
use std::ffi::{CStr, CString};
use std::process::Command;
use std::thread;
struct Executor;
impl Executor {
    fn launch(&self, path: &str) -> i32 {
        if path.contains("/tmp/malicious") {
            return 1;
        }
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("echo Execution")
            .env("PATH", path)
            .spawn()
            .expect("failed to spawn process");
        let status = child.wait().expect("process wasn't running");
        status.code().unwrap_or(-1)
    }
}
unsafe fn convert_raw(ptr: *mut i8) -> String {
    let c_str = CStr::from_ptr(ptr);
    c_str.to_string_lossy().into_owned()
}
fn acquire_untrusted() -> String {
    env::var("MAL_INPUT").unwrap_or_else(|_| String::from("/malicious/path"))
}
fn main() {
    let exec = Executor{};
    let untrusted_input = acquire_untrusted();
    let mut raw_bytes = untrusted_input.clone().into_bytes();
    raw_bytes.push(0); 
    let boxed = raw_bytes.into_boxed_slice();
    let ptr = Box::into_raw(boxed) as *mut i8;
    let converted = unsafe { convert_raw(ptr) };
    let handle = thread::spawn(move || {
        exec.launch(&converted)
    });
    let result = handle.join().expect("Thread panicked");
    println!("Process exited with code: {}", result);
}
----------------------------------------------------
