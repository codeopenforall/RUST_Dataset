use std::env;
use std::ffi::{CString, CStr};
use std::ptr;
use std::sync::Arc;
use std::thread;
extern "C" {
    fn snprintf(buf: *mut i8, size: usize, format: *const i8, ...) -> i32;
}
fn process(input: &str) -> String {
    let fixed_format = CString::new("%s").expect("CString conversion failed for fixed format");
    let c_input = CString::new(input).expect("CString conversion failed for user input");
    let mut buffer = [0i8; 256];
    unsafe {
        let _ = snprintf(buffer.as_mut_ptr(), buffer.len(), fixed_format.as_ptr(), c_input.as_ptr());
        CStr::from_ptr(buffer.as_ptr()).to_string_lossy().into_owned()
    }
}
fn run_app() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        return;
    }
    let user_input = args[1].clone();
    let shared = Arc::new(user_input);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || process(&shared_clone));
    let result = handle.join().unwrap();
    println!("{}", result);
}
fn main() {
    run_app();
}
