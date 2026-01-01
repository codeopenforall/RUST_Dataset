extern crate libc;
use std::env;
use std::ffi::CString;
use std::thread;
struct Printer;
impl Printer {
    fn output(&self, input: &str) {
        unsafe {
            let c_format = CString::new(input).expect("Failed to create CString");
            libc::printf(c_format.as_ptr());
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 {
        args[1].as_str()
    } else {
        "Hello, world!"
    };
    let instance = Printer;
    let handle = thread::spawn(move || {
        instance.output(user_input);
    });
    handle.join().unwrap();
}
