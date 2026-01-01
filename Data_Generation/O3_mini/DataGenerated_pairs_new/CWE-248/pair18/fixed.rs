//////////////////////////////
// Fixed Code for CWE-248
//////////////////////////////
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;
use std::thread;

#[no_mangle]
pub extern "C" fn service(input: *const c_char) -> i32 {
    // Wrap the potentially panicking code with catch_unwind.
    let result = panic::catch_unwind(|| {
        // SAFETY: The caller must ensure 'input' is a valid pointer.
        unsafe {
            let c_str = CStr::from_ptr(input);
            let mut s = c_str.to_string_lossy().into_owned();
            s.push_str("!");
            let _ = *s.as_ptr();
        }
        // Simulate an error situation that would otherwise panic.
        panic!("handled panic")
    });
    // Instead of propagating the panic, return an error code.
    match result {
        Ok(_val) => 0,  // Normal execution path (not reached in this test)
        Err(_)    => -1, // Error code indicating failure was caught.
    }
}

fn main() {
    let input = CString::new("test").unwrap();
    // Even with threading the panic is now caught.
    let handle = thread::spawn(move || unsafe { service(input.as_ptr()) });
    let ret = handle.join().unwrap();
    println!("Completed with return code: {}", ret);
}