use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Shared {
    value: i32,
    flag: bool, 
}
impl Shared {
    fn new(val: i32) -> Self {
        Shared { value: val, flag: false }
    }
    fn update(&mut self, delta: i32) -> Result<i32, &'static str> {
        if self.value < 0 {
            return Err("negative value detected");
        }
        unsafe {
            let ptr = &mut self.value as *mut i32;
            *ptr = *ptr + delta;
        }
        Ok(self.value)
    }
}
fn perform_task(shared: Arc<Mutex<Shared>>, delta: i32) -> Result<i32, &'static str> {
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let mut lock = shared_clone.lock().unwrap();
        lock.flag = true;
    });
    handle.join().unwrap();
    let mut lock = shared.lock().unwrap();
    lock.update(delta)
}
fn main() {
    let shared = Arc::new(Mutex::new(Shared::new(10)));
    match perform_task(shared, 5) {
        Ok(val) => println!("Operation succeeded with result: {}", val),
        Err(err) => println!("Operation failed: {}", err),
    }
}
