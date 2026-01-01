//////////////////////////////////////////////
// Vulnerable Code: Memory Unsafe Processing
//////////////////////////////////////////////
#![allow(dead_code)]
use std::ptr;

pub fn do_process(input: &[u8]) -> Result<u32, &'static str> {
    // Fixed-size internal buffer (16 bytes)
    let mut buf = [0u8; 16];

    // UNSAFE: Copies input bytes into the fixed buffer without checking bounds.
    // If input.len() > 16, this will write out-of-bounds and corrupt memory.
    unsafe {
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), input.len());
    }

    // Compute the sum of the bytes in the fixed buffer.
    let sum: u32 = buf.iter().map(|&b| b as u32).sum();
    Ok(sum)
}

fn main() {
    let input = vec![1u8; 20]; // 20 bytes input to trigger out-of-bound write.
    match do_process(&input) {
        Ok(s) => println!("Sum computed: {}", s),
        Err(e) => println!("Error encountered: {}", e),
    }
}