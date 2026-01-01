//////////////////////////////////////////////////////////////
// Vulnerability Code Sample for CWE-131:
// This implementation mistakenly computes the size of the dynamic buffer 
// using the size of a reference to the input string instead of its actual 
// length. It misuses mem::size_of_val to determine the allocation size, which 
// on 64-bit targets yields 16 bytes rather than the proper byte length of the string.
// In a concurrent context the unsafe block and manual memory management amplify the risk.
//////////////////////////////////////////////////////////////
use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;

struct Data {
    raw: *mut u8,
    cap: usize,
}

impl Data {
    // The problem: computing capacity as mem::size_of_val(&s) + 1 instead of s.len()+1.
    // This miscalculation (and the added byte) results in copying more bytes than exist in s.
    unsafe fn create(s: &str) -> Self {
        // Incorrect calculation: mem::size_of_val(&s) returns the size in bytes of the slice (pointer + length)
        // instead of the actual number of bytes in s.
        let cap = mem::size_of_val(&s) + 1;
        let layout = Layout::from_size_align(cap, 1).unwrap();
        let raw = alloc(layout);
        // Copy cap bytes from s (which is too many because s.len() is typically much less).
        ptr::copy_nonoverlapping(s.as_ptr(), raw, cap);
        Data { raw, cap }
    }

    // Retrieve a string slice from the internal buffer.
    unsafe fn as_text(&self) -> &str {
        // Use the stored capacity minus terminator.
        let len = self.cap - 1;
        let slice = std::slice::from_raw_parts(self.raw, len);
        std::str::from_utf8(slice).unwrap()
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.cap, 1).unwrap();
            dealloc(self.raw, layout);
        }
    }
}

fn main() {
    // Simulate concurrent processing in threads.
    let input = "Hello, world!";
    let shared = Arc::new(input.to_string());
    let mut threads = vec![];
    for _ in 0..4 {
        let data_ref = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            unsafe {
                let item = Data::create(&data_ref);
                // This assertion might spuriously fail or lead to undefined behavior because of the size miscalculation.
                assert_eq!(item.as_text(), "Hello, world!", "Mismatch in buffer content");
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}