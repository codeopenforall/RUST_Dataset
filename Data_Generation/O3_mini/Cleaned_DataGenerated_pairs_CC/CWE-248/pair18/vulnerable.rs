use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::thread;
#[no_mangle]
pub extern "C" fn service(input: *const c_char) -> i32 {
    unsafe {
        let c_str = CStr::from_ptr(input);
        let mut s = c_str.to_string_lossy().into_owned();
        s.push_str("!");
        let _ = *s.as_ptr();
    }
    panic!("uncaught panic error")
}
fn main() {
    let input = CString::new("test").unwrap();
    let handle = thread::spawn(move || unsafe { service(input.as_ptr()) });
    let _ = handle.join().unwrap();
}
