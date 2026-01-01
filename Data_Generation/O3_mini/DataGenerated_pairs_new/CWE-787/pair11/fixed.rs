use std::ptr;

struct Processor;

impl Processor {
    // Transforms the input slice into a new vector using safe capacity calculations.
    // The vector is allocated with the full input length, and the unsafe block is
    // used solely for pointer-based copying within valid bounds.
    pub fn transform(input: &[i32]) -> Vec<i32> {
        let n = input.len();
        // Properly allocate buffer with capacity equal to n.
        let mut buffer: Vec<i32> = Vec::with_capacity(n);
        unsafe {
            // Set the length to n so that writes remain within bounds.
            buffer.set_len(n);
            let ptr = buffer.as_mut_ptr();
            for i in 0..n {
                // Write safely within the allocated memory.
                ptr.add(i).write(input[i]);
            }
        }
        buffer
    }
}

fn main() {
    let data = vec![10, 20, 30, 40];
    let result = Processor::transform(&data);
    println!("Result: {:?}", result);
}