/////////////////////////////////////////////////////////////
// Vulnerable Code: Externally-Controlled Format String Issue //
/////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};
use std::os::raw::{c_int};
extern "C" {
    fn sprintf(buffer: *mut i8, format: *const i8, ...) -> c_int;
}

struct Processor {
    buffer: Arc<Mutex<Vec<i8>>>,
}

impl Processor {
    fn new() -> Self {
        // allocate buffer of 256 bytes initialized to zero (i8)
        let buf = vec![0_i8; 256];
        Self {
            buffer: Arc::new(Mutex::new(buf)),
        }
    }

    // Method that uses an externally provided format string directly,
    // which is dangerous if the format string is crafted by an attacker.
    fn process(&self, fmt: &str) {
        let fmt_c = CString::new(fmt).expect("CString::new failed");
        // The safe data parameter to be inserted in the format string
        let data_c = CString::new("data").expect("CString::new failed");
        let mut guard = self.buffer.lock().unwrap();
        unsafe {
            // Vulnerability: Using an externally provided format string directly.
            // If fmt contains unexpected format specifiers (e.g. "%s%s"), it may lead
            // to reading uninitialized memory.
            sprintf(guard.as_mut_ptr(), fmt_c.as_ptr(), data_c.as_ptr());
        }
    }

    // Utility to get the resulting output as a Rust String.
    fn output(&self) -> String {
        let guard = self.buffer.lock().unwrap();
        // Interpret the buffer as a null-terminated C string.
        unsafe { CStr::from_ptr(guard.as_ptr()).to_string_lossy().into_owned() }
    }
}

fn main() {
    let proc_inst = Processor::new();
    let shared_inst = Arc::new(proc_inst);

    // Spawn threads simulating concurrent use, where the format string comes
    // externally (e.g. from network or user input). In a real-world scenario,
    // different threads might supply their own user input.
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let local = Arc::clone(&shared_inst);
            thread::spawn(move || {
                // Here the external input is used directly.
                // An attacker might provide a malicious format string.
                local.process("%s%s");
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("Output Buffer: {:?}", shared_inst.output());
}