/* 
This Rust code demonstrates a potential out‐of‐bounds read vulnerability (CWE-125).
It uses unsafe pointer arithmetic inside a concurrent setup. 
The Processor type exposes a method to read a byte at an index from an internally shared vector.
When an out‐of‐bounds index is provided, the unsafe block performs unchecked pointer arithmetic,
potentially reading adjacent memory.
*/
use std::sync::Arc;
use std::thread;

pub mod app {
    use super::*;
    
    pub struct Processor {
        data: Arc<Vec<u8>>,
    }
    
    impl Processor {
        pub fn new(v: Vec<u8>) -> Self {
            Self { data: Arc::new(v) }
        }
        
        // This method returns an Option<u8> but always wraps the result in Some(..)
        // even if the index is out-of-bound. It uses unchecked pointer arithmetic.
        pub fn compute(&self, idx: usize) -> Option<u8> {
            unsafe {
                // POTENTIAL FLAW: No bounds check; may read out-of-bound memory.
                Some(*self.data.as_ptr().add(idx))
            }
        }
    }
    
    pub fn run_app() {
        // Create a vector of three elements.
        let vec_data = vec![10u8, 20, 30];
        let proc_obj = Processor::new(vec_data);
        let shared_proc = Arc::new(proc_obj);
        
        // Spawn several threads to concurrently use the Processor.
        let mut handles = vec![];
        for _ in 0..4 {
            let proc_clone = shared_proc.clone();
            handles.push(thread::spawn(move || {
                // Intentionally accessing index equal to vec length, which is out-of-bound.
                proc_clone.compute(3)
            }));
        }
        
        // Join threads and print the results.
        for handle in handles {
            // This may trigger undefined behavior or return an unexpected value.
            let result = handle.join().expect("Thread panicked");
            println!("Result: {:?}", result);
        }
    }
}

fn main() {
    app::run_app();
}