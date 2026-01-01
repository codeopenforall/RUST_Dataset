use std::thread;
struct Processor {
    callback: fn(&str) -> String,
}
impl Processor {
    pub fn execute(&self, input: &str) -> Option<String> {
        Some(unsafe { (self.callback)(input) })
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
        let _ = proc.execute("trigger");
    });
    handle.join().unwrap();
}
