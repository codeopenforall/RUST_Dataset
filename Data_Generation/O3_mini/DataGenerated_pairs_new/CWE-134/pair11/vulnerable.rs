/* 
This Rust program reads a string passed via command-line argument and uses it as the format string
in a formatting macro inside an unsafe block. The code spawns a thread to exercise concurrent use. 
The unsafe block first reinterprets the inner String as a &str without validation and then passes it 
directly into a format! macro as the formatting template. If the user input (the format string) contains 
format specifiers that do not match the provided arguments, this can lead to unexpected panics or behavior 
due to externally-controlled formatting – a CWE-134 instance.
*/
use std::env;
use std::thread;
use std::slice;
use std::str;

struct Data {
    input: String,
}

trait Executor {
    fn execute(&self) -> String;
}

impl Executor for Data {
    fn execute(&self) -> String {
        // Unsafe block performing unchecked conversion and using user input as format string.
        unsafe {
            let ptr = self.input.as_ptr();
            let len = self.input.len();
            let user_format = str::from_utf8_unchecked(slice::from_raw_parts(ptr, len));
            // CWE-134: Externally-controlled format string vulnerability.
            format!(user_format, "foo", "bar")
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <format-string>");
        return;
    }
    let data = Data { input: args[1].clone() };
    let handle = thread::spawn(move || data.execute());
    let res = handle.join().unwrap();
    println!("{}", res);
}