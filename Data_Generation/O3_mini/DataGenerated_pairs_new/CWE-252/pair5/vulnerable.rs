use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Data {
    count: Arc<Mutex<u32>>,
}

impl Data {
    // This method updates the shared counter using an unsafe raw pointer.
    // It returns an error if the new value exceeds the allowed threshold.
    unsafe fn update(&self, new_val: u32) -> Result<(), &'static str> {
        let mut guard = self.count.lock().unwrap();
        let raw_ptr = &mut *guard as *mut u32;
        if new_val > 100 {
            return Err("Value exceeds maximum allowed threshold");
        }
        // Unsafe memory write simulating a system call or FFI operation.
        ptr::write(raw_ptr, new_val);
        Ok(())
    }
}

fn run() {
    let data = Data { count: Arc::new(Mutex::new(0)) };
    let data_clone = Data { count: Arc::clone(&data.count) };

    let handle = thread::spawn(move || {
        unsafe {
            // VULNERABILITY: The return value from update is ignored.
            let _ = data_clone.update(150);
        }
    });
    handle.join().unwrap();

    // The final value remains unchanged even though an erroneous update was attempted.
    let final_value = *data.count.lock().unwrap();
    println!("Final value: {}", final_value);
}

fn main() {
    run();
}