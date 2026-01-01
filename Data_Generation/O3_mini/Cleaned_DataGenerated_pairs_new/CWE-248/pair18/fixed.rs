use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;
use std::thread;
#[no_mangle]
pub extern "C" fn service(input: *const c_char) -> i32 {
    let result = panic::catch_unwind(|| {
        unsafe {
            let c_str = CStr::from_ptr(input);
            let mut s = c_str.to_string_lossy().into_owned();
            s.push_str("!");
            let _ = *s.as_ptr();
        }
        panic!("handled panic")
    });
    match result {
        Ok(_val) => 0,  
        Err(_)    => -1, 
    }
}
fn main() {
    let input = CString::new("test").unwrap();
    let handle = thread::spawn(move || unsafe { service(input.as_ptr()) });
    let ret = handle.join().unwrap();
    println!("Completed with return code: {}", ret);
}
