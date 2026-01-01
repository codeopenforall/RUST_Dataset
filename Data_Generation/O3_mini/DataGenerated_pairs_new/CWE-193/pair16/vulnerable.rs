////////////////////////////////////////////
// Vulnerability Example in Rust (CWE-193)
// Off-by-One Error in vector length management using unsafe code
////////////////////////////////////////////
use std::ptr;
use std::thread;

struct Processor;

impl Processor {
    // Processes a slice of u32, incrementing each element by 1.
    // WARNING: Contains an off-by-one error.
    pub fn process(data: &[u32]) -> Vec<u32> {
        let n = data.len();
        let mut out: Vec<u32> = Vec::with_capacity(n);
        unsafe {
            let ptr_out = out.as_mut_ptr();
            for i in 0..n {
                // Write each incremented element to the uninitialized memory.
                ptr_out.add(i).write(data[i].wrapping_add(1));
            }
            // FLAW: Off-by-one error: set_len(n + 1) writes one extra (uninitialized) element.
            out.set_len(n + 1);
        }
        out
    }
}

fn main() {
    let data = vec![10, 20, 30, 40];
    // Spawn multiple threads to mimic real-life concurrent usage.
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