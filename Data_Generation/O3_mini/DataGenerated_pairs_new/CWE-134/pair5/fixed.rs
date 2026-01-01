/////////////////////////////////////////////////////////////
// Corrected Code: Securing the Format String Handling     //
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
        let buf = vec![0_i8; 256];
        Self {
            buffer: Arc::new(Mutex::new(buf)),
        }
    }

    // In this safe version, the externally provided format string is ignored.
    // A constant safe format string is used instead.
    fn process(&self, _fmt: &str) {
        // Fixed: use a constant format string so any malicious content is not used.
        let safe_fmt = CString::new("%s").expect("CString::new failed");
        let data_c = CString::new("data").expect("CString::new failed");
        let mut guard = self.buffer.lock().unwrap();
        unsafe {
            sprintf(guard.as_mut_ptr(), safe_fmt.as_ptr(), data_c.as_ptr());
        }
    }

    fn output(&self) -> String {
        let guard = self.buffer.lock().unwrap();
        unsafe { CStr::from_ptr(guard.as_ptr()).to_string_lossy().into_owned() }
    }
}

fn main() {
    let proc_inst = Processor::new();
    let shared_inst = Arc::new(proc_inst);

    let threads: Vec<_> = (0..2)
        .map(|_| {
            let local = Arc::clone(&shared_inst);
            thread::spawn(move || {
                // Even if external input is malicious, the safe processing ignores it.
                local.process("%s%s");
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }

    println!("Output Buffer: {:?}", shared_inst.output());
}