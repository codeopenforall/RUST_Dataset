//////////////////////
// Vulnerable version
//////////////////////
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;
use std::thread;

extern "C" {
    // FFI to C library function - note: using a user-controlled format string.
    fn snprintf(
        buf: *mut c_char,
        size: usize,
        fmt: *const c_char,
        ...
    ) -> i32;
}

struct Processor {}

impl Processor {
    fn new() -> Processor {
        Processor {}
    }

    // This function uses the user-provided string directly as a format string;
    // if the string contains unexpected format specifiers, it may read unintended memory.
    fn process(&self, user_input: &str) -> String {
        // Convert the user input to a C string.
        let cstr = CString::new(user_input).unwrap();
        let mut buffer = [0u8; 256];
        unsafe {
            // Vulnerability: user input is treated as a format specifier.
            // No additional arguments are provided for any format specifiers.
            snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                cstr.as_ptr(),
            );
        }
        // Convert the buffer (up to first NUL) back to a Rust string.
        let output = CString::new(&buffer[..]).unwrap_or_else(|_| CString::new("Error").unwrap());
        output.to_string_lossy().into_owned()
    }
}

fn main() {
    let proc = Arc::new(Processor::new());
    // Spawn a thread to simulate concurrent logging.
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        // Malicious input that includes format specifiers. In a real exploit this
        // might leak memory or crash the process.
        let res = proc_clone.process("%x %x %x");
        println!("{}", res);
    });
    handle.join().unwrap();
}