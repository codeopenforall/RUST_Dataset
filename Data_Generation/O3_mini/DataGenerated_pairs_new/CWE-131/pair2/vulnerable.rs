//////////////////////////////////////////////////////////////
// A realistic program that processes text via a shared
// concurrent object. It uses unsafe memory operations to
// allocate and copy string data. However, the computed buffer
// size is incorrect: Instead of using the length of the input,
// it mistakenly uses the size of the String slice pointer,
// leading to a buffer overflow if the input is longer than a fixed
// pointer size (typically 16 bytes on 64-bit systems).
//////////////////////////////////////////////////////////////
use std::alloc::{alloc, dealloc, Layout, handle_alloc_error};
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

trait ProcessorTrait {
    fn compute(&self, input: &str) -> Vec<u8>;
}

struct DataProcessor;

impl ProcessorTrait for DataProcessor {
    fn compute(&self, input: &str) -> Vec<u8> {
        unsafe {
            // Incorrect buffer size calculation: using size_of_val on the slice reference
            // instead of the actual byte length of the input string.
            let miscalc = std::mem::size_of_val(&input) + 1; // Vulnerability occurs here
            let layout = Layout::from_size_align(miscalc, std::mem::align_of::<u8>()).unwrap();
            let mem_ptr = alloc(layout);
            if mem_ptr.is_null() {
                handle_alloc_error(layout);
            }
            // Copy the input bytes (if input.len() > miscalc, this causes buffer overflow)
            ptr::copy_nonoverlapping(input.as_ptr(), mem_ptr, input.len());
            // Write the terminating zero byte
            *mem_ptr.add(input.len()) = 0;
            let slice = std::slice::from_raw_parts(mem_ptr, input.len() + 1);
            let result = slice.to_vec();
            dealloc(mem_ptr, layout);
            result
        }
    }
}

fn main() {
    // Shared concurrent data processor used by multiple threads.
    let processor = Arc::new(Mutex::new(DataProcessor));
    // Prepare an input string longer than 16 bytes to trigger the vulnerability.
    let malicious = "A".repeat(100);
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let proc_clone = Arc::clone(&processor);
            let input = malicious.clone();
            thread::spawn(move || {
                let proc = proc_clone.lock().unwrap();
                let output = proc.compute(&input);
                // The below assertions may not hold due to incorrect allocation.
                assert_eq!(output.len(), input.len() + 1);
                assert_eq!(output[input.len()], 0);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
    println!("Completed processing in vulnerable version.");
}