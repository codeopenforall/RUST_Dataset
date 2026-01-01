//////////////////////////////
// Fixed Example in Rust
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
    // The fixed unsafe function validates the new value BEFORE performing the update.
    unsafe fn modify(&self, new_val: i32) -> Result<(), &'static str> {
        if new_val < 0 {
            return Err("value cannot be negative");
        }
        let mut cfg = self.config.lock().unwrap();
        let ptr = &mut *cfg as *mut Config;
        (*ptr).value = new_val;
        Ok(())
    }

    // Execution now properly handles the returned Result.
    fn execute(&self, new_val: i32) {
        unsafe {
            // Propagate errors so that invalid updates are not silently applied.
            self.modify(new_val).unwrap();
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

    // Attempt an invalid update. The error is now handled immediately.
    let res = unsafe { proc_inst.modify(-5) };
    if res.is_err() {
        println!("Negative update rejected safely");
    }

    // Final invariant check: the configuration value remains non-negative.
    let final_val = cfg.lock().unwrap().value;
    if final_val < 0 {
        panic!("Invariant violation: negative value detected");
    }

    println!("Final value: {}", final_val);
}