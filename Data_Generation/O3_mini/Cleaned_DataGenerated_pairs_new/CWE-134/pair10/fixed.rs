use std::env;
use std::ffi::{CString, CStr};
use std::sync::{Arc, Mutex};
use std::thread;
extern "C" {
    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
}
fn process(user: &str) -> String {
    let mut buffer = [0u8; 256];
    let c_user = CString::new(user).expect("Conversion to CString failed");
    let c_fixed_format = CString::new("%s").expect("Conversion to CString failed");
    unsafe {
        snprintf(
            buffer.as_mut_ptr() as *mut i8,
            buffer.len(),
            c_fixed_format.as_ptr(),
            c_user.as_ptr(),
        );
    }
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
    c_str.to_string_lossy().into_owned()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal_input" };
    let shared_result = Arc::new(Mutex::new(String::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let input_clone = input.to_string();
        let shared_clone = Arc::clone(&shared_result);
        let handle = thread::spawn(move || {
            let res = process(&input_clone);
            let mut data = shared_clone.lock().unwrap();
            *data = res;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    let final_result = shared_result.lock().unwrap();
    println!("Result: {}", *final_result);
}
