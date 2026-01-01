use std::env;
use std::ffi::CString;
use std::thread;
extern crate libc;
struct Printer;
impl Printer {
    fn output(&self, input: &str) {
        let raw = CString::new(input).unwrap();
        unsafe {
            libc::printf(raw.as_ptr());
        }
    }
}
fn execute() {
    let args: Vec<String> = env::args().collect();
    let user_str = if args.len() > 1 { &args[1] } else { "Hello, %s!" };
    let printer = Printer;
    let handle = thread::spawn(move || {
        printer.output(user_str);
    });
    handle.join().unwrap();
}
fn main() {
    execute();
}
