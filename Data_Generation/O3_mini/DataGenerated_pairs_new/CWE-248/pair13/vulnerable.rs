use std::thread;

struct Processor {
    callback: fn(&str) -> String,
}

impl Processor {
    // The execute method calls the callback inside an unsafe block.
    // If the callback panics (for input "trigger"), the panic is not caught.
    pub fn execute(&self, input: &str) -> Option<String> {
        // Unsafe call without any panic handling.
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
        // When "trigger" is passed, callback_fn panics.
        let _ = proc.execute("trigger");
    });
    // The thread join unwrap will propagate the panic from the spawned thread.
    handle.join().unwrap();
}