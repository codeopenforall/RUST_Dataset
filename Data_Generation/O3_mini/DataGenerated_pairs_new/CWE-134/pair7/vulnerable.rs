///////////////////// Vulnerable Code /////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

trait Formatter {
    // Executes a formatting operation using an externally controlled format string.
    // Unsafe due to unchecked format string usage.
    unsafe fn execute(&self, fmt: &str) -> String;
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
    unsafe fn execute(&self, fmt: &str) -> String {
        let count = *self.counter.lock().unwrap();
        // POTENTIAL FLAW (CWE-134): Directly passing an externally-controlled format string,
        // which may trigger a panic or unexpected behavior if malicious placeholders are present.
        format!(fmt, count)
    }
}

fn main() {
    let core = Core::new();
    core.update(123);
    let core_shared = Arc::new(core);

    // Retrieve external input as the format string.
    // E.g., a malicious user could supply: "Exploit: {} {}" to trigger a panic.
    let fmt_arg = env::args().nth(1).unwrap_or_else(|| "Counter: {}".to_string());
    let core_clone = Arc::clone(&core_shared);

    let handle = thread::spawn(move || {
        unsafe {
            let msg = core_clone.execute(&fmt_arg);
            println!("{}", msg);
        }
    });

    handle.join().unwrap();
}
///////////////////////////////////////////////////////////////