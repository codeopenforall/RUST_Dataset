use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Resource {
    // Use safe shared ownership and locking to manage the integer.
    data: Arc<Mutex<i32>>,
}

impl Resource {
    // Properly initialize the resource wrapped in Arc and Mutex.
    fn new(val: i32) -> Self {
        Resource {
            data: Arc::new(Mutex::new(val)),
        }
    }

    // Safely reads the value with proper synchronization.
    fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}

// Public function that safely accesses the shared resource.
// It spawns a thread to concurrently read the resource value.
// Both the main thread and the spawned thread obtain consistent results.
pub fn execute() -> i32 {
    let res = Resource::new(123);
    let data_clone = Arc::clone(&res.data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        *data_clone.lock().unwrap()
    });
    let main_val = res.read();
    let thread_val = handle.join().unwrap();
    // Ensure that both reads are equal (the expected value).
    assert_eq!(main_val, thread_val);
    main_val
}

fn main() {
    let result = execute();
    println!("Value: {}", result);
}