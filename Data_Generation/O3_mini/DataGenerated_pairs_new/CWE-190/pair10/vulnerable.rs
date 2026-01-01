use std::thread;
use std::sync::Arc;

struct Processor {
    factor: u32,
}

impl Processor {
    // Processes a slice of u32 numbers unsafely,
    // accumulating using wrapping arithmetic.
    fn compute(&self, data: &[u32]) -> u32 {
        let mut sum: u32 = 0;
        unsafe {
            let ptr = data.as_ptr();
            for i in 0..data.len() {
                // Add each element using wrapping arithmetic
                sum = sum.wrapping_add(*ptr.add(i));
            }
            // Perform multiplication in an unchecked context,
            // which may overflow without detection.
            sum = sum * self.factor;
        }
        sum
    }
}

fn run_calc() -> u64 {
    // Create input data designed to trigger an overflow
    // when the individual sums and multiplication are computed as u32.
    let numbers = Arc::new(vec![u32::MAX, 1]);
    let proc = Arc::new(Processor { factor: 2 });
    
    let data_clone = Arc::clone(&numbers);
    let proc_clone = Arc::clone(&proc);
    let handle1 = thread::spawn(move || {
        // Process the first element in a separate thread.
        proc_clone.compute(&data_clone[0..1])
    });
    
    let data_clone2 = Arc::clone(&numbers);
    let proc_clone2 = Arc::clone(&proc);
    let handle2 = thread::spawn(move || {
        // Process the second element concurrently.
        proc_clone2.compute(&data_clone2[1..2])
    });
    
    let part1 = handle1.join().unwrap();
    let part2 = handle2.join().unwrap();
    
    // Combine the two partial results using wrapping addition.
    let combined = part1.wrapping_add(part2);
    // Cast to u64 before returning, ignoring that the intermediate u32 arithmetic wrapped.
    combined as u64
}

fn main() {
    let result = run_calc();
    println!("Result: {}", result);
}