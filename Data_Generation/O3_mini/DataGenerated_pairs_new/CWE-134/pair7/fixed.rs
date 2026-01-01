///////////////////// Corrected Code ///////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

trait Formatter {
    // Executes a safe formatting operation by treating external input as plain text.
    fn execute(&self, user_message: &str) -> String;
}

struct Core {
    counter: Arc<Mutex<i32>>,
}

impl Core {
    fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }
    
    fn update(&self, val: i32) {
        let mut cnt = self.counter.lock().unwrap();
        *cnt = val;
    }
}

impl Formatter for Core {
    fn execute(&self, user_message: &str) -> String {
        let count = *self.counter.lock().unwrap();
        // FIX: Do not use the external input as a format string.
        // Instead, incorporate it as plain text in a static format context.
        format!("{}; Counter: {}", user_message, count)
    }
}

fn main() {
    let core = Core::new();
    core.update(123);
    let core_shared = Arc::new(core);

    // Retrieve external input text. It is now handled as data and not as a format specifier.
    let user_input = env::args().nth(1).unwrap_or_else(|| "Counter value is provided".to_string());
    let core_clone = Arc::clone(&core_shared);

    let handle = thread::spawn(move || {
        let msg = core_clone.execute(&user_input);
        println!("{}", msg);
    });

    handle.join().unwrap();
}
///////////////////////////////////////////////////////////////