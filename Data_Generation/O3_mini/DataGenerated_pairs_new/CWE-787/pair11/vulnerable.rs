use std::ptr;

struct Processor;

impl Processor {
    // Transforms the input slice into a new vector using unsafe operations.
    // Incorrectly calculates the capacity, allocating one element less than needed,
    // then writes all input elements, resulting in an out‐of‐bounds write.
    pub fn transform(input: &[i32]) -> Vec<i32> {
        let n = input.len();
        // Incorrect capacity: one element less than n.
        let capacity = n.saturating_sub(1);
        let mut buffer: Vec<i32> = Vec::with_capacity(capacity);
        unsafe {
            // Set the length to capacity even though we attempt to write n elements.
            buffer.set_len(capacity);
            let ptr = buffer.as_mut_ptr();
            // This loop iterates n times.
            for i in 0..n {
                // When i reaches capacity, writing goes out-of-bounds.
                ptr.add(i).write(input[i]);
            }
        }
        buffer
    }
}

fn main() {
    // Use a fixed input that triggers the out-of-bounds write (n=4 and capacity=3).
    let data = vec![10, 20, 30, 40];
    let result = Processor::transform(&data);
    println!("Result: {:?}", result);
}