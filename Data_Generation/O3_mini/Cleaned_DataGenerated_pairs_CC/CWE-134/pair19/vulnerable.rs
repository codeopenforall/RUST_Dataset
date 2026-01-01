use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;
extern "C" {
    pub fn printf(format: *const i8, ...) -> i32;
}
pub struct Processor {
    data: Arc<Mutex<String>>,
}
impl Processor {
    pub fn new(s: String) -> Self {
        Processor {
            data: Arc::new(Mutex::new(s)),
        }
    }
    pub fn run(&self, input: &str) -> i32 {
        let mut locked = self.data.lock().unwrap();
        *locked = input.to_owned();
        let c_input = CString::new(locked.clone()).unwrap();
        unsafe {
            printf(c_input.as_ptr())
        }
    }
}
pub fn process(input: &str) -> i32 {
    let proc_main = Processor::new(String::new());
    let proc_thread = Processor {
        data: Arc::clone(&proc_main.data),
    };
    let input_owned = input.to_owned();
    let handle = thread::spawn(move || {
        proc_thread.run(&input_owned)
    });
    let main_ret = proc_main.run(input);
    let thread_ret = handle.join().unwrap();
    main_ret + thread_ret
}
fn main() {
    let malicious = "Hello %x";
    let printed = process(malicious);
    println!("Printed {} characters", printed);
}
