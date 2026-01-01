//////////////////////////////////////
// Vulnerable Code for CWE-134
//////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::os::raw::c_char;
use std::ffi::CString;

extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}

struct Data {
    info: String,
}

trait Displayable {
    fn show(&self);
}

impl Displayable for Data {
    fn show(&self) {
        // Here the user-controlled string is taken directly and used as a format string.
        // This is done in an unsafe block with an external C function.
        let raw_input = self.info.clone();
        unsafe {
            // POTENTIAL FLAW: Passing the externally controlled string as a format string.
            let fmt = CString::new(raw_input).expect("CString creation failed");
            // The external function interprets any format specifiers found in the input.
            printf(fmt.as_ptr());
        }
    }
}

fn handle_message(user_str: String) -> Result<(), String> {
    let data = Data { info: user_str };
    let shared = Arc::new(Mutex::new(data));
    let threads: Vec<_> = (0..3).map(|_| {
        let cloned = shared.clone();
        thread::spawn(move || {
            let locked = cloned.lock().expect("Locking failed");
            locked.show();
        })
    }).collect();

    for t in threads {
        t.join().map_err(|_| "Thread join failed".to_string())?;
    }
    Ok(())
}

fn main() {
    // Simulated user input containing format specifiers.
    let user_input = "%s%s%s";
    // Passing the malicious input to the processing function.
    handle_message(user_input.to_string()).expect("Processing failed");
}