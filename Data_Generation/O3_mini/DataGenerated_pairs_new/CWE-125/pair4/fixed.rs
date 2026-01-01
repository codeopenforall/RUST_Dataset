/* 
This corrected Rust code addresses the out‐of‐bounds read vulnerability (CWE-125)
by performing a proper bounds check using the safe API of slices.
The Processor type exposes the same method signature, but now it uses Vec::get to safely
access the element. Concurrent threads calling an out‐of‐bounds index now receive None,
preventing unsafe memory access.
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
        
        // Now performs a safe bounds check.
        pub fn compute(&self, idx: usize) -> Option<u8> {
            self.data.get(idx).cloned()
        }
    }
    
    pub fn run_app() {
        let vec_data = vec![10u8, 20, 30];
        let proc_obj = Processor::new(vec_data);
        let shared_proc = Arc::new(proc_obj);
        
        let mut handles = vec![];
        for _ in 0..4 {
            let proc_clone = shared_proc.clone();
            handles.push(thread::spawn(move || {
                // Using the same index; safe access returns None for out-of-bound.
                proc_clone.compute(3)
            }));
        }
        
        for handle in handles {
            let result = handle.join().expect("Thread panicked");
            match result {
                Some(val) => println!("Result: {}", val),
                None => println!("Out-of-bounds access safely detected"),
            }
        }
    }
}

fn main() {
    app::run_app();
}