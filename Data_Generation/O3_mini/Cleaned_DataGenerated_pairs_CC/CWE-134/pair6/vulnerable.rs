extern crate libc;
use std::env;
use std::ffi::CString;
use std::thread;
struct Processor;
impl Processor {
    fn run(&self, fmt: &str) {
        let fmt_owned = fmt.to_owned();
        let handle = thread::spawn(move || {
            let cstr = CString::new(fmt_owned).unwrap();
            unsafe {
                libc::printf(cstr.as_ptr(), 42);
            }
        });
        let _ = handle.join();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <format_string>", args[0]);
        return;
    }
    let fmt = &args[1];
    let proc_inst = Processor;
    proc_inst.run(fmt);
}
