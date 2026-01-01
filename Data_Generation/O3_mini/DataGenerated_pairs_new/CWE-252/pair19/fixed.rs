use std::sync::{Arc, Mutex};
use std::thread;

struct Holder {
    data: i32,
}

impl Holder {
    // This method first validates the input and only performs the unsafe update if the input is valid.
    fn modify(&mut self, val: i32) -> Result<(), &'static str> {
        if val < 0 {
            return Err("negative value not allowed");
        }
        unsafe {
            let ptr: *mut i32 = &mut self.data;
            *ptr = val;
        }
        Ok(())
    }
}

fn main() {
    let holder = Arc::new(Mutex::new(Holder { data: 0 }));
    let holder_clone = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        let mut h = holder_clone.lock().unwrap();
        // Proper error handling: the update is allowed to fail, and the error is propagated.
        h.modify(-10).expect("update failed due to negative value");
    });
    // Even if the thread panics, join will propagate the panic.
    let _ = handle.join();
    let final_value = holder.lock().unwrap().data;
    println!("Final value: {}", final_value);
}