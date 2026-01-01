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
            let miscalc = std::mem::size_of_val(&input) + 1; 
            let layout = Layout::from_size_align(miscalc, std::mem::align_of::<u8>()).unwrap();
            let mem_ptr = alloc(layout);
            if mem_ptr.is_null() {
                handle_alloc_error(layout);
            }
            ptr::copy_nonoverlapping(input.as_ptr(), mem_ptr, input.len());
            *mem_ptr.add(input.len()) = 0;
            let slice = std::slice::from_raw_parts(mem_ptr, input.len() + 1);
            let result = slice.to_vec();
            dealloc(mem_ptr, layout);
            result
        }
    }
}
fn main() {
    let processor = Arc::new(Mutex::new(DataProcessor));
    let malicious = "A".repeat(100);
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let proc_clone = Arc::clone(&processor);
            let input = malicious.clone();
            thread::spawn(move || {
                let proc = proc_clone.lock().unwrap();
                let output = proc.compute(&input);
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
