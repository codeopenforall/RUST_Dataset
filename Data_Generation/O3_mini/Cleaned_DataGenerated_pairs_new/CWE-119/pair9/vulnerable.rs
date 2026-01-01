#![allow(dead_code)]
use std::ptr;
pub fn do_process(input: &[u8]) -> Result<u32, &'static str> {
    let mut buf = [0u8; 16];
    unsafe {
        ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), input.len());
    }
    let sum: u32 = buf.iter().map(|&b| b as u32).sum();
    Ok(sum)
}
fn main() {
    let input = vec![1u8; 20]; 
    match do_process(&input) {
        Ok(s) => println!("Sum computed: {}", s),
        Err(e) => println!("Error encountered: {}", e),
    }
}
