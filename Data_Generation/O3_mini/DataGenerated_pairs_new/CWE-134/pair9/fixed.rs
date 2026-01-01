/*
This corrected version avoids using the external user-controlled string as the format specifier.
Instead, it uses a fixed format string and treats the user input simply as data.
The overall structure remains similar, including concurrency via a spawned thread, so that the API remains
comparable to the vulnerable version.
*/
use std::thread;

struct Processor;

impl Processor {
    // Securely format the string by using a fixed format pattern.
    fn process(&self, user_data: &str, value: i32) -> Option<String> {
        // FIX: Do not use user_data as a format string. Instead, incorporate it as data.
        Some(format!("{}: {}", user_data, value))
    }
}

fn run_parallel(input: String, val: i32) -> Option<String> {
    let proc = Processor;
    let handle = thread::spawn(move || {
        proc.process(&input, val)
    });
    handle.join().ok().flatten()
}

fn main() {
    let user_input = "%x %x %x %x";
    let output = run_parallel(user_input.to_string(), 42).unwrap_or_else(|| "error".to_string());
    println!("Result: {}", output);
}