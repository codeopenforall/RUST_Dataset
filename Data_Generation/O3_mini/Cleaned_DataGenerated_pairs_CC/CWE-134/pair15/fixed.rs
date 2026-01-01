use std::env;
use std::ffi::CString;
use std::thread;
extern crate libc;
struct Printer;
impl Printer {
    fn output(&self, input: &str) {
        let c_input = CString::new(input).unwrap();
        let fmt = CString::new("%s").unwrap();
        unsafe {
            libc::printf(fmt.as_ptr(), c_input.as_ptr());
        }
    }
}
fn execute() {
    let args: Vec<String> = env::args().collect();
    let user_str = if args.len() > 1 { &args[1] } else { "Hello, user!" };
    let printer = Printer;
    let handle = thread::spawn(move || {
        printer.output(user_str);
    });
    handle.join().unwrap();
}
fn main() {
    execute();
}
