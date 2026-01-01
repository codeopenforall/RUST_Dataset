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
    fn process(&self, fmt: &str) {
        let fmt_c = CString::new(fmt).expect("CString::new failed");
        let data_c = CString::new("data").expect("CString::new failed");
        let mut guard = self.buffer.lock().unwrap();
        unsafe {
            sprintf(guard.as_mut_ptr(), fmt_c.as_ptr(), data_c.as_ptr());
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
                local.process("%s%s");
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    println!("Output Buffer: {:?}", shared_inst.output());
}
