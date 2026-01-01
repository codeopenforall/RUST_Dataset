use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
mod app {
    use super::*;
    pub struct Item {
        secret: u32,
        public: u32,
    }
    impl Item {
        pub fn new(flag: bool) -> Self {
            let mut uninit = MaybeUninit::<Item>::uninit();
            unsafe {
                let ptr = uninit.as_mut_ptr();
                (*ptr).secret = 42;
                if flag {
                    (*ptr).public = 128;
                }
                uninit.assume_init()
            }
        }
        pub fn sum(&self) -> u32 {
            self.secret + self.public
        }
    }
    pub fn execute() {
        let item = Item::new(false);
        let shared = Arc::new(Mutex::new(item));
        let shared_clone = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let obj = shared_clone.lock().unwrap();
            obj.sum()
        });
        let sum_main = {
            let obj = shared.lock().unwrap();
            obj.sum()
        };
        let sum_thread = handle.join().unwrap();
        println!("Sum from main: {}, Sum from thread: {}", sum_main, sum_thread);
    }
}
fn main() {
    app::execute();
}
