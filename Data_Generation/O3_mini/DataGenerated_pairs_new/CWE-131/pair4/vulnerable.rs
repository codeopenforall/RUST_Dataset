//////////////////// Vulnerable Code ////////////////////
// This code computes the size of a fixed buffer using std::mem::size_of_val,
// but it allocates an array with no additional space for a null terminator.
// In an unsafe block the code copies exactly the size of the buffer from an input string
// and then unconditionally writes a null terminator after the end of the allocated buffer.
// This miscalculation results in a buffer overrun, which can corrupt adjacent memory.
// The code also involves a thread spawn to mimic a realistic concurrent usage pattern.

use std::mem;
use std::ptr;
use std::thread;

pub fn process() -> Vec<u8> {
    // The input is exactly 16 bytes in length.
    let input = String::from("1234567890123456");
    // Allocate a buffer of 16 bytes without room for the terminator.
    let mut buf = [0u8; 16];

    unsafe {
        let size = mem::size_of_val(&buf); // size = 16
        // Copy exactly 16 bytes from input into buf.
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), size);
        // Incorrectly write a null terminator past the end of the buffer.
        // This writes at index 16, which is out-of-bounds for a 16-element array.
        *buf.as_mut_ptr().add(size) = 0;
    }

    // Return a vector constructed from the buffer.
    // Note: Even though the out-of-bounds write exhibits undefined behaviour,
    // the program might compile and run without immediate panic or error.
    buf.to_vec()
}

fn run_app() {
    let data = process();
    println!("Buffer content: {:?}", data);
}

fn main() {
    let handle = thread::spawn(|| {
        run_app();
    });
    handle.join().unwrap();
}