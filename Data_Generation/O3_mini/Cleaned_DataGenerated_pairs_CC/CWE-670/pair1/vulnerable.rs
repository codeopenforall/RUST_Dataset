use std::sync::{Arc, Mutex};
use std::thread;
struct Controller {
    flag: bool,
    counter: i32,
}
impl Controller {
    pub fn new() -> Self {
        Controller { flag: true, counter: 0 }
    }
    pub fn update(&mut self) {
        unsafe {
            let flag_ptr: *mut bool = &mut self.flag;
            if *flag_ptr {
                self.counter = self.counter.wrapping_sub(1);
            } else {
                self.counter = self.counter.wrapping_add(1);
            }
            if self.counter < 0 {
                *flag_ptr = true;
            } else {
                *flag_ptr = false;
            }
        }
    }
}
fn main() {
    let controller = Arc::new(Mutex::new(Controller::new()));
    let mut handles = vec![];
    for _ in 0..10 {
        let ctrl = Arc::clone(&controller);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut c = ctrl.lock().unwrap();
                c.update();
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    let c = controller.lock().unwrap();
    println!("Final counter: {}, flag: {}", c.counter, c.flag);
}
