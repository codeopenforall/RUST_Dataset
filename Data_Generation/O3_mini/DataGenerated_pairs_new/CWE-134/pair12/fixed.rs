extern crate libc;
use std::env;
use std::thread;

// A helper structure that prints messages using safe Rust formatting.
struct Printer;

impl Printer {
    // This method safely prints the user-provided message by using a fixed format string.
    fn output(&self, input: &str) {
        // Instead of using the user input as a format string, we embed it as data.
        println!("{}", input);
    }
}

fn main() {
    // Retrieve external input from command-line arguments.
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 {
        args[1].as_str()
    } else {
        "Hello, world!"
    };

    let instance = Printer;
    // Spawn a concurrent thread to simulate multi-threaded usage.
    let handle = thread::spawn(move || {
        instance.output(user_input);
    });
    handle.join().unwrap();
}