use std::thread;
struct Processor;
impl Processor {
    fn process(&self, user_data: &str, value: i32) -> Option<String> {
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
