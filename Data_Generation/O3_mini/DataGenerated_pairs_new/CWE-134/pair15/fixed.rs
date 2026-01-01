//////////////////////////
// Corrected Code Sample
//////////////////////////
use std::env;
use std::ffi::CString;
use std::thread;
extern crate libc;

struct Printer;

impl Printer {
    // This corrected method uses a constant format string ("%s")
    // and passes the user input as an argument to be printed. This
    // ensures that any format specifiers in the user input are treated
    // as literal text.
    fn output(&self, input: &str) {
        // Convert the user input into a C-compatible string.
        let c_input = CString::new(input).unwrap();
        // Use a constant format string that expects a single string argument.
        let fmt = CString::new("%s").unwrap();
        unsafe {
            libc::printf(fmt.as_ptr(), c_input.as_ptr());
        }
    }
}

fn execute() {
    let args: Vec<String> = env::args().collect();
    // If no argument is provided, use a benign default input.
    let user_str = if args.len() > 1 { &args[1] } else { "Hello, user!" };
    let printer = Printer;
    // Spawn a thread to simulate concurrency.
    let handle = thread::spawn(move || {
        printer.output(user_str);
    });
    handle.join().unwrap();
}

fn main() {
    execute();
}