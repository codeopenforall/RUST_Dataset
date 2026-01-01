use std::sync::{Arc, Mutex};
use std::thread;
struct Controller {
    total: i32,
}
impl Controller {
    unsafe fn unsafe_adjustment(&mut self, increment: i32) -> i32 {
        if self.total == 0 {
            1 
        } else {
            self.total += increment;
            0 
        }
    }
    fn adjust(&mut self, increment: i32) -> Result<(), &'static str> {
        unsafe {
            let res = self.unsafe_adjustment(increment);
            if res == 0 {
                Ok(())
            } else {
                Err("Adjustment failed")
            }
        }
    }
}
fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut c = Controller { total: start };
    c.adjust(increment)?;
    Ok(c.total)
}
fn main() {
    let controller = Arc::new(Mutex::new(Controller { total: 1 }));
    let controller_clone = Arc::clone(&controller);
    let handle = thread::spawn(move || {
        let mut c = controller_clone.lock().unwrap();
        c.adjust(10).unwrap();
    });
    handle.join().unwrap();
    println!("Total: {}", controller.lock().unwrap().total);
}
