use std::thread;
use std::panic;
struct Processor {
    callback: fn(&str) -> String,
}
impl Processor {
    pub fn execute(&self, input: &str) -> Option<String> {
        match panic::catch_unwind(|| unsafe { (self.callback)(input) }) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}
fn callback_fn(msg: &str) -> String {
    if msg == "trigger" {
        panic!("Simulated panic occurred!");
    }
    format!("Processed: {}", msg)
}
fn main() {
    let proc = Processor { callback: callback_fn };
    let handle = thread::spawn(move || {
        let result = proc.execute("trigger");
        match result {
            Some(val) => println!("{}", val),
            None => println!("Recovered from panic"),
        }
    });
    handle.join().unwrap();
}
