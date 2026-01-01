use std::sync::{Arc, Mutex};
use std::thread;

struct Holder {
    data: i32,
}

impl Holder {
    // This function performs an unsafe update regardless of the input value.
    // It returns an error if the given value is negative, but still proceeds with the update.
    fn modify(&mut self, val: i32) -> Result<(), &'static str> {
        // Determine if the value is invalid.
        let res = if val < 0 { Err("negative value not allowed") } else { Ok(()) };
        // Unsafe block performing direct memory update.
        unsafe {
            let ptr: *mut i32 = &mut self.data;
            *ptr = val;
        }
        res
    }
}

fn main() {
    let holder = Arc::new(Mutex::new(Holder { data: 0 }));
    let holder_clone = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        let mut h = holder_clone.lock().unwrap();
        // The return value is ignored: if the update fails, the error is not handled.
        let _ = h.modify(-10); // Vulnerability: Unchecked return value.
    });
    handle.join().unwrap();
    let final_value = holder.lock().unwrap().data;
    println!("Final value: {}", final_value);
}