------------------ vulnerable.rs ------------------
#![allow(unused_unsafe)]
use std::ffi::{CStr, CString};
        if path.contains("/tmp/malicious") {
            return 1;
        }
unsafe fn convert_raw(ptr: *mut i8) -> String {
    let c_str = CStr::from_ptr(ptr);
    c_str.to_string_lossy().into_owned()
}
fn acquire_untrusted() -> String {
    env::var("MAL_INPUT").unwrap_or_else(|_| String::from("/malicious/path"))
}
    let untrusted_input = acquire_untrusted();
    let mut raw_bytes = untrusted_input.clone().into_bytes();
    raw_bytes.push(0); 
    let boxed = raw_bytes.into_boxed_slice();
    let ptr = Box::into_raw(boxed) as *mut i8;
    let converted = unsafe { convert_raw(ptr) };
        exec.launch(&converted)
