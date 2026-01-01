////////////////////////////////////////////////////////////////////////////////////////////////////
// This code simulates a configuration object that is created using unsafe uninitialized memory.
// It spawns multiple threads which use the configuration. However, one field (flag) is never properly
// initialized, leading to undefined behavior (CWE-908: Use of Uninitialized Resource).
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    value: i32,
    flag: bool,
}

impl Data {
    // Unsafe constructor that leaves 'flag' uninitialized.
    unsafe fn create_uninit() -> Self {
        let mut uninit = MaybeUninit::<Self>::uninit();
        // Initialize only the 'value' field.
        (*uninit.as_mut_ptr()).value = 42;
        // flag is intentionally left uninitialized to mimic a real-world mistake.
        uninit.assume_init()
    }
}

fn compute() -> (i32, bool) {
    // Shared configuration wrapped in a Mutex for safe concurrent access.
    let shared = Arc::new(Mutex::new(Data { value: 0, flag: false }));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            // Unsafe usage: Creating a Data instance with uninitialized 'flag'.
            let unsafe_data = unsafe { Data::create_uninit() };
            let mut data = shared_clone.lock().unwrap();
            data.value = data.value.wrapping_add(unsafe_data.value);
            // Use the uninitialized flag value. Its indeterminate state can lead to undefined behavior.
            if unsafe_data.flag {
                data.flag = true;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let final_data = shared.lock().unwrap();
    (final_data.value, final_data.flag)
}

fn main() {
    let result = compute();
    println!("Result: {:?}", result);
}