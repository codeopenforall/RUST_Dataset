////////////////////// Fixed Code //////////////////////
use std::thread;
use std::sync::Arc;

struct Processor {
    data: Vec<u32>,
    scale: u32,
}

impl Processor {
    // The corrected method computes the scaled average with proper operator precedence:
    // By performing multiplication first and then division, it avoids premature truncation.
    fn compute(&self) -> u32 {
        let sum: u32 = self.data.iter().sum();
        let count = self.data.len() as u32;
        // Removed unsafe block as it is unnecessary.
        let result = sum * self.scale / count;
        result
    }
}

fn main() {
    let proc_obj = Arc::new(Processor { data: vec![1, 2], scale: 100 });
    let mut threads = vec![];
    for _ in 0..2 {
        let proc_clone = Arc::clone(&proc_obj);
        let handle = thread::spawn(move || {
            let output = proc_clone.compute();
            println!("Computed result: {}", output);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}