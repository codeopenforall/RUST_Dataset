//////////////////////////////////////////////////////////////
// Vulnerable implementation for demonstration purposes.
// This version miscalculates a memory buffer's size using
// mem::size_of::<&str>() instead of the actual byte length.
//////////////////////////////////////////////////////////////

use std::mem;
use std::ptr;
use std::thread;

struct Communicator;

impl Communicator {
    // Transmit copies the input string into a newly allocated buffer.
    // Vulnerability: It calculates the buffer size using mem::size_of::<&str>()
    // which yields a fixed pointer size instead of the actual input byte length.
    fn transmit(&self, input: &str) -> Box<[u8]> {
        // Incorrect buffer size: using size of the &str pointer instead of input.len()
        let buffer_size = mem::size_of::<&str>(); // Vulnerable line
        let mut buffer = vec![0u8; buffer_size].into_boxed_slice();

        unsafe {
            // Copying the input bytes into buffer without checking boundaries.
            // If input.len() > buffer_size, out-of-bound memory writes will occur.
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), input.len());
        }
        buffer
    }
}

fn main() {
    let comm = Communicator;
    // Spawn a thread to mimic concurrent use-pattern.
    let handle = thread::spawn(move || {
        // Provide deliberately long input to trigger overflow.
        let data = "A".repeat(50);
        let buf = comm.transmit(&data);
        // Compute a simple checksum from the buffer data.
        buf.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
    });
    let checksum = handle.join().expect("Thread panic");
    println!("Resulting checksum: {}", checksum);
}