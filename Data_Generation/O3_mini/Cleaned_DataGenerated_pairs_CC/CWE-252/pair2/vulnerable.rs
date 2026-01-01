use std::sync::{Arc, Mutex};
use std::thread;
struct Processor {
    config: Arc<Mutex<Config>>,
}
struct Config {
    value: i32,
}
impl Processor {
    unsafe fn modify(&self, new_val: i32) -> Result<(), &'static str> {
        let mut cfg = self.config.lock().unwrap();
        let ptr = &mut *cfg as *mut Config;
        (*ptr).value = new_val;
        if new_val < 0 {
            return Err("value cannot be negative");
        }
        Ok(())
    }
    fn execute(&self, new_val: i32) {
        unsafe {
            let _ = self.modify(new_val);
        }
    }
}
fn main() {
    let cfg = Arc::new(Mutex::new(Config { value: 0 }));
    let proc_inst = Processor { config: cfg.clone() };
    let t1_config = cfg.clone();
    let t1 = thread::spawn(move || {
        let proc_obj = Processor { config: t1_config };
        proc_obj.execute(10);
    });
    t1.join().unwrap();
    proc_inst.execute(-5);
    let final_val = cfg.lock().unwrap().value;
    if final_val < 0 {
        panic!("Invariant violation: negative value detected");
    }
    println!("Final value: {}", final_val);
}
