extern crate libc;
use std::env;
use std::ffi::CString;
use std::thread;
struct Processor;
impl Processor {
    fn run(&self, input: &str) {
        let safe_input = CString::new(input).unwrap();
        let safe_fmt = CString::new("User input: %s, fixed number: %d\n").unwrap();
        let handle = thread::spawn(move || {
            unsafe {
                libc::printf(safe_fmt.as_ptr(), safe_input.as_ptr(), 42);
            }
        });
        let _ = handle.join();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <user_input>", args[0]);
        return;
    }
    let input = &args[1];
    let proc_inst = Processor;
    proc_inst.run(input);
}
