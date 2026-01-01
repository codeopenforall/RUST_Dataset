//
// This code intentionally misuses uninitialized memory.
// It defines a module "app" with a struct "Item" that is partially
// initialized. When the condition is false, the 'public' field is
// left uninitialized and later used in arithmetic operations,
// which can lead to undefined behavior.
//
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};

mod app {
    use super::*;
    
    pub struct Item {
        secret: u32,
        public: u32,
    }

    impl Item {
        // The constructor conditionally initializes the 'public' field.
        // When 'flag' is false, 'public' remains uninitialized.
        pub fn new(flag: bool) -> Self {
            let mut uninit = MaybeUninit::<Item>::uninit();
            unsafe {
                let ptr = uninit.as_mut_ptr();
                (*ptr).secret = 42;
                if flag {
                    (*ptr).public = 128;
                }
                // If flag is false, 'public' is left uninitialized.
                uninit.assume_init()
            }
        }

        pub fn sum(&self) -> u32 {
            self.secret + self.public
        }
    }

    // The function 'execute' demonstrates a concurrency usage.
    // It spawns a thread that accesses the same Item concurrently.
    pub fn execute() {
        // Force the problematic branch: pass false so 'public' remains uninitialized.
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