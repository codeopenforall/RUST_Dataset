//////////////////////////////
// Vulnerable Code for CWE-248
//////////////////////////////
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::thread;

#[no_mangle]
pub extern "C" fn service(input: *const c_char) -> i32 {
    // SAFETY: We assume the pointer is valid. However, no panic handling is in place.
    unsafe {
        let c_str = CStr::from_ptr(input);
        // Convert C string into Rust String
        let mut s = c_str.to_string_lossy().into_owned();
        // Append extra data (simulating data manipulation)
        s.push_str("!");
        // Perform an unsafe read (dummy usage to mimic real-life unsafe code)
        let _ = *s.as_ptr();
    }
    // Vulnerability: Panicking without catching unwind across a thread/FFI boundary
    panic!("uncaught panic error")
}

fn main() {
    let input = CString::new("test").unwrap();
    // Spawn a new thread that calls the FFI-exposed service routine
    let handle = thread::spawn(move || unsafe { service(input.as_ptr()) });
    // Joining the thread will unwrap the panic, causing abnormal termination.
    let _ = handle.join().unwrap();
}