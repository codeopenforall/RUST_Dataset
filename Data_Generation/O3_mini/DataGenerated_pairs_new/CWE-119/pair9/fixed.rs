//////////////////////////////////////////////
// Corrected Code: Memory Safe Processing
//////////////////////////////////////////////
#![allow(dead_code)]
use std::ptr;

pub fn do_process(input: &[u8]) -> Result<u32, &'static str> {
    // Fixed-size internal buffer (16 bytes)
    let mut buf = [0u8; 16];

    // Check that the input size does not exceed buffer capacity.
    if input.len() > buf.len() {
        return Err("Input too large");
    }

    // SAFETY: The length-check guarantees that the copy does not exceed the buffer.
    unsafe {
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), input.len());
    }

    // Compute the sum of the bytes in the fixed buffer.
    let sum: u32 = buf.iter().map(|&b| b as u32).sum();
    Ok(sum)
}

fn main() {
    let input = vec![1u8; 20]; // 20 bytes input; expected to be rejected.
    match do_process(&input) {
        Ok(s) => println!("Sum computed: {}", s),
        Err(e) => println!("Error encountered: {}", e),
    }
}