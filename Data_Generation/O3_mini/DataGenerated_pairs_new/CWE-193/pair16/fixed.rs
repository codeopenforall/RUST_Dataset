////////////////////////////////////////////
// Corrected Example in Rust
// Off-by-One Error Fixed in vector length management using unsafe code
////////////////////////////////////////////
use std::ptr;
use std::thread;

struct Processor;

impl Processor {
    // Processes a slice of u32, incrementing each element by 1.
    // Correctly sets the length to match the number of written elements.
    pub fn process(data: &[u32]) -> Vec<u32> {
        let n = data.len();
        let mut out: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr_out = out.as_mut_ptr();
            for i in 0..n {
                ptr_out.add(i).write(data[i].wrapping_add(1));
            }
            // FIX: Accurately set the length to n elements.
            out.set_len(n);
        }
        out
    }
}

fn main() {
    let data = vec![10, 20, 30, 40];
    // Spawn multiple threads to simulate concurrent execution.
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let data_thread = data.clone();
            thread::spawn(move || {
                let result = Processor::process(&data_thread);
                println!("Result: {:?}", result);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}