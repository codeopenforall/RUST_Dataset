//////////////////////////////
// Vulnerable Example in Rust
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    config: Arc<Mutex<Config>>,
}

struct Config {
    value: i32,
}

impl Processor {
    // This unsafe function performs a memory update even if the new value is invalid.
    // It writes the value first, then checks for an error condition.
    unsafe fn modify(&self, new_val: i32) -> Result<(), &'static str> {
        let mut cfg = self.config.lock().unwrap();
        let ptr = &mut *cfg as *mut Config;
        // Unconditionally update the value (even if it is negative)
        (*ptr).value = new_val;
        if new_val < 0 {
            return Err("value cannot be negative");
        }
        Ok(())
    }

    // The execution method ignores the return value from modify.
    fn execute(&self, new_val: i32) {
        // Vulnerability: the Result returned by modify is dropped without checking.
        unsafe {
            let _ = self.modify(new_val);
        }
    }
}

fn main() {
    let cfg = Arc::new(Mutex::new(Config { value: 0 }));
    let proc_inst = Processor { config: cfg.clone() };

    // Spawn a thread that performs a valid update.
    let t1_config = cfg.clone();
    let t1 = thread::spawn(move || {
        let proc_obj = Processor { config: t1_config };
        proc_obj.execute(10);
    });
    t1.join().unwrap();

    // In the main thread, a negative update is performed.
    // The returned error is unchecked, so the invalid update is applied.
    proc_inst.execute(-5);

    // Invariant check: a negative value violates the contract.
    let final_val = cfg.lock().unwrap().value;
    if final_val < 0 {
        panic!("Invariant violation: negative value detected");
    }

    println!("Final value: {}", final_val);
}