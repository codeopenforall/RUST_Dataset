//////////////////////////////////////////////////////////////
// Fixed implementation for demonstration purposes.
// This version correctly calculates the required buffer size
// based on the input string's byte length and allocates extra
// space for a null terminator.
//////////////////////////////////////////////////////////////

use std::ptr;
use std::thread;

struct Communicator;

impl Communicator {
    // Transmit copies the input string into a new buffer.
    // It now correctly computes the buffer size using input.len()
    // plus an extra byte for a null terminator.
    fn transmit(&self, input: &str) -> Box<[u8]> {
        // Correctly calculate buffer size: actual byte length plus one.
        let buffer_size = input.len() + 1;
        let mut buffer = vec![0u8; buffer_size].into_boxed_slice();

        unsafe {
            // Copy the input data into the buffer.
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), input.len());
            // Set explicit null terminator at the end.
            *buffer.as_mut_ptr().add(input.len()) = 0;
        }
        buffer
    }
}

fn main() {
    let comm = Communicator;
    // Spawn a thread to simulate similar concurrent processing.
    let handle = thread::spawn(move || {
        // Use the same input that previously triggered overflow.
        let data = "A".repeat(50);
        let buf = comm.transmit(&data);
        // Compute checksum only over the copied data (ignoring the terminator).
        buf[..data.len()].iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
    });
    let checksum = handle.join().expect("Thread panic");
    println!("Resulting checksum: {}", checksum);
}