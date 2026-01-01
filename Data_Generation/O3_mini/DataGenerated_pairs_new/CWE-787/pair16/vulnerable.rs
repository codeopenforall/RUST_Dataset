////////////////////////////////////////////////////////////////
// This program intentionally performs an out‐of‐bounds memory write
// using unsafe pointer arithmetic combined with manual length setting.
// It defines a processor that, when run in a separate thread, sums a set
// of numbers stored in a vector. The unsafe block writes one element past
// the allocated space. This can lead to memory corruption or unpredictable
// behavior, violating CWE-787: Out-of-bounds Write.
////////////////////////////////////////////////////////////////
use std::thread;

fn compute_value() -> i32 {
    let mut data = vec![1, 2, 3, 4, 5];              // Line 4
    unsafe {
        // Writing outside the allocated buffer.
        let ptr = data.as_mut_ptr();                // Line 7
        *ptr.add(5) = 999;                           // Line 8: Unsafe out-of-bounds write!
        // Manually set the length to include the new element,
        // even though it lies outside the originally allocated range.
        data.set_len(6);                             // Line 9
    }
    data.iter().sum()
}

struct Worker;

impl Worker {
    fn process(&self) -> i32 {
        compute_value()
    }
}

fn main() {
    let handler = Worker;
    // Spawn a thread to simulate concurrency.
    let handle = thread::spawn(move || {
        handler.process()
    });

    let result = handle.join().unwrap();
    println!("Total: {}", result);
}