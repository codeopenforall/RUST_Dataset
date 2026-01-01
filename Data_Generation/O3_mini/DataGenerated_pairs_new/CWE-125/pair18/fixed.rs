////////////////////////
// Fixed Code File    //
////////////////////////
use std::sync::Arc;
use std::thread;

struct Processor {
    data: Vec<u8>,
}

impl Processor {
    fn new(data: Vec<u8>) -> Self {
        Processor { data }
    }

    // This method safely reads a value from a buffer by checking bounds.
    // If the index is out-of-range, None is returned.
    fn extract(&self, index: usize) -> Option<u8> {
        if index < self.data.len() {
            Some(self.data[index])
        } else {
            None
        }
    }
}

fn run_task(proc: Arc<Processor>, idx: usize) -> Option<u8> {
    proc.extract(idx)
}

fn main() {
    let data = vec![10, 20, 30]; // Buffer length = 3
    let proc = Arc::new(Processor::new(data));
    let index = 3; // Index is now properly checked. Out-of-bound accesses yield None.
    
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || run_task(proc_clone, index));
    let result = handle.join().unwrap();
    
    // In the fixed version, the out-of-bound access is detected and handled safely.
    println!("Extracted value: {:?}", result);
}