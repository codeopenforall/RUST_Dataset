//////////////////////
// Fixed version
//////////////////////
use std::sync::Arc;
use std::thread;

struct Processor {}

impl Processor {
    fn new() -> Processor {
        Processor {}
    }

    // In the safe version, the user input is not interpreted as a format string.
    // Instead, it is inserted into a fixed format string.
    fn process(&self, user_input: &str) -> String {
        // Safely print the input by explicitly formatting it as data.
        format!("{}", user_input)
    }
}

fn main() {
    let proc = Arc::new(Processor::new());
    // Spawn a thread to simulate concurrent processing.
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || {
        // Even if the input contains format specifiers, they will not be interpreted.
        let res = proc_clone.process("%x %x %x");
        println!("{}", res);
    });
    handle.join().unwrap();
}