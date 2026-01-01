use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Clone, Copy, Debug, PartialEq)]
enum Access {
    Allowed,
    Blocked,
}
struct Handler {
    flag: Arc<Mutex<Access>>,
}
impl Handler {
    fn new() -> Self {
        Handler {
            flag: Arc::new(Mutex::new(Access::Blocked)),
        }
    }
    fn update(&self, input: i32) {
        unsafe {
            let new_state = match input > 100 {
                true => Access::Allowed,
                false => Access::Blocked,
            };
            let mut current = self.flag.lock().unwrap();
            *current = new_state;
        }
    }
    fn check(&self) -> bool {
        let current = self.flag.lock().unwrap();
        match *current {
            Access::Allowed => true,
            Access::Blocked => false,
        }
    }
}
fn main() {
    let handler = Handler::new();
    let shared_handler = Arc::new(handler);
    let mut handles = vec![];
    for _ in 0..5 {
        let clone = Arc::clone(&shared_handler);
        let t = thread::spawn(move || {
            clone.update(150);
        });
        handles.push(t);
    }
    for t in handles {
        t.join().unwrap();
    }
    if shared_handler.check() {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}
