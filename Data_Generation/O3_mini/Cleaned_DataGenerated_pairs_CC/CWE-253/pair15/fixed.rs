use std::sync::{Arc, Mutex};
use std::thread;
struct Container {
    number: i32,
}
impl Container {
    unsafe fn try_update(&mut self, new_number: i32) -> Result<(), &'static str> {
        if new_number == 0 {
            return Err("Zero is not allowed");
        }
        self.number = new_number;
        Ok(())
    }
}
fn concurrent_update(shared: Arc<Mutex<Container>>, new_number: i32) -> bool {
    let mut threads = vec![];
    for _ in 0..4 {
        let data = shared.clone();
        threads.push(thread::spawn(move || {
            unsafe {
                let mut guard = data.lock().unwrap();
                let ret = guard.try_update(new_number);
                if ret.is_ok() {
                    true
                } else {
                    false
                }
            }
        }));
    }
    let mut success = false;
    for handle in threads {
        success |= handle.join().unwrap();
    }
    success
}
fn main() {
    let shared = Arc::new(Mutex::new(Container { number: 42 }));
    if concurrent_update(shared.clone(), 0) {
        println!("Concurrent update reported success.");
    } else {
        println!("Concurrent update reported failure (fixed behavior).");
    }
}
