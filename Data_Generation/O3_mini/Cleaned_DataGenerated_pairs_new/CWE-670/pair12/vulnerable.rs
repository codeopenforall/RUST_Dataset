use std::sync::{Arc, Mutex};
use std::thread;
struct Engine {
    flag: Arc<Mutex<u32>>,
}
impl Engine {
    fn new() -> Self {
        Engine { flag: Arc::new(Mutex::new(0)) }
    }
    fn process(&self, val: u32) -> Result<u32, &'static str> {
        match val {
            2 => {
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 10;
                Ok(*cnt)
            },
            1 | _ => {
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    let bad_ptr = base_ptr.offset(1);
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt)
                }
            },
        }
    }
}
fn main() {
    let core = Engine::new();
    match core.process(1) {
        Ok(res) => println!("Outcome: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}
