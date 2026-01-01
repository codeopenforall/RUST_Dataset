//////////////////// Fixed Code ////////////////////
// The corrected code allocates enough space for both the input data and the null terminator.
// It explicitly subtracts one byte from the total allocated size when copying from the input,
// ensuring that the null terminator is written within bounds. The structure and concurrency
// patterns remain unchanged to mimic real-world production code.

use std::mem;
use std::ptr;
use std::thread;

pub fn process() -> Vec<u8> {
    // The input is exactly 16 bytes in length.
    let input = String::from("1234567890123456");
    // Allocate a buffer with an extra byte for the null terminator.
    let mut buf = [0u8; 17];

    unsafe {
        // Compute the number of bytes to copy: total allocated minus one for the terminator.
        let size = mem::size_of_val(&buf) - 1; // size = 16
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), size);
        // Safely write the null terminator within bounds.
        *buf.as_mut_ptr().add(size) = 0;
    }

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