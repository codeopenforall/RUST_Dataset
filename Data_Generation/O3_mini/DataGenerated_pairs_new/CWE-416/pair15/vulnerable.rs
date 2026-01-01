use std::thread;

trait Operation {
    fn process(&self) -> i32;
}

struct Handler;

impl Operation for Handler {
    fn process(&self) -> i32 {
        // Allocate an integer on the heap.
        let boxed = Box::new(42);
        // Convert it to a raw pointer, relinquishing ownership.
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            // Free the allocated memory by reconstructing and dropping the Box.
            // This line frees the data, but we still hold raw_ptr.
            drop(Box::from_raw(raw_ptr));  // Vulnerability: memory freed here (line 16)
            // Simulate concurrent activity that might reuse the freed memory.
            let handle = thread::spawn(|| {
                let _dummy = vec![0u8; 1024];
            });
            handle.join().unwrap();
            // Dereference the dangling raw pointer, causing a use-after-free.
            (*raw_ptr)  // Vulnerability: use-after-free occurs here (line 23)
        }
    }
}

pub fn compute() -> i32 {
    let proc = Handler;
    proc.process()
}

fn main() {
    let result = compute();
    println!("Result: {}", result);
}