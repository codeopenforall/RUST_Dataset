use std::thread;
use std::panic;

struct Processor {
    callback: fn(&str) -> String,
}

impl Processor {
    // The execute method now wraps the unsafe callback call within catch_unwind,
    // preventing panics from propagating across thread boundaries.
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
        // The panic from callback_fn is caught, so we safely get a None.
        let result = proc.execute("trigger");
        match result {
            Some(val) => println!("{}", val),
            None => println!("Recovered from panic"),
        }
    });
    // The thread join now succeeds as the panic is contained.
    handle.join().unwrap();
}