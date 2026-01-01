use std::sync::{Arc, Mutex};
use std::thread;
unsafe fn external_process(value: i32) -> i32 {
    if value > 100 {
        1
    } else {
        0
    }
}
struct Engine {
    flag: Arc<Mutex<bool>>,
}
impl Engine {
    fn new() -> Self {
        Engine {
            flag: Arc::new(Mutex::new(false)),
        }
    }
    fn process(&self, value: i32) -> bool {
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            let res = unsafe { external_process(value) };
            if res != 0 {
                let mut f = flag.lock().unwrap();
                *f = true;
            } else {
                let mut f = flag.lock().unwrap();
                *f = false;
            }
        });
        handle.join().unwrap();
        *self.flag.lock().unwrap()
    }
}
fn main() {
    let engine = Engine::new();
    let outcome = engine.process(150);
    if outcome {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}
