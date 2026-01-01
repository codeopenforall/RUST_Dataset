/* 
   This code defines a Buffer structure that holds a fixed‐size array and a guard value.
   The processing routine uses an unsafe pointer arithmetic loop that iterates one index too far,
   reading a value that lies outside the “data” array. Due to #[repr(C)], the guard field is laid
   out immediately after the array in memory, so the loop inadvertently includes the guard.
   This is analogous to an out‐of‐bounds read vulnerability (CWE-125) in real applications.
*/
use std::sync::Arc;
use std::thread;

#[repr(C)]
struct Buffer {
    data: [i32; 4],
    // Guard field that is not meant to be processed
    guard: i32,
}

impl Buffer {
    // The processing function uses unsafe pointer arithmetic and unchecked indexing.
    // It intentionally loops from 0 to data.len() inclusive.
    fn calculate(&self) -> i32 {
        let n = self.data.len();
        let mut sum = 0;
        // Unsafe block that performs out-of-bound read when i == n.
        unsafe {
            let ptr = self.data.as_ptr();
            // Vulnerability: iterates from 0 to n inclusive, so when i == n, get_unchecked reads from memory not part of the array.
            for i in 0..=n {
                // NOTE: When i == n this reads memory past the end of data, potentially the guard field.
                sum += *ptr.add(i);
            }
        }
        sum
    }
}

fn main() {
    // Create a Buffer with data and an unintended guard value.
    // The correct sum of data is expected to be 10, but the vulnerability includes the guard (42).
    let buf = Buffer { data: [1, 2, 3, 4], guard: 42 };
    // Wrap in an Arc and spawn a thread to mimic concurrency in real-world usage.
    let shared = Arc::new(buf);
    let shared_clone = Arc::clone(&shared);

    let handler = thread::spawn(move || {
        // This thread calls the processing routine.
        shared_clone.calculate()
    });

    // Wait for thread to join and obtain the result.
    let res = handler.join().unwrap();
    // In vulnerable code, the extra out-of-bound read adds the guard value.
    println!("Computed sum: {}", res);
}