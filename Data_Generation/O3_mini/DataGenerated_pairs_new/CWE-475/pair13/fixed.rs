#!/usr/bin/env rust
// This revised version eliminates the buffer overflow vulnerability by ensuring that the number
// of bytes copied never exceeds the destination buffer capacity. The code calculates the correct
// copy length based on the source data slice length and allocates exactly enough memory for the data.
//
// Compile with: rustc fixed.rs

use std::ptr;
use std::thread;

pub struct App {}

impl App {
    // Correctly performs a copy of data with proper bounds-checking.
    pub fn process(&self, data: &[u8]) -> Vec<u8> {
        // Allocate a buffer with capacity exactly equal to the input data length.
        let mut buf = Vec::with_capacity(data.len());
        unsafe {
            // Safe copy: only copy the number of bytes which were allocated.
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), data.len());
            buf.set_len(data.len());
        }
        buf
    }
}

fn main() {
    let app = App {};
    let input = vec![65u8; 10]; // input: 10 bytes of ASCII 'A'
    let output = app.process(&input);
    println!("Output: {:?}", output);

    // Simulate concurrent execution to mimic real-world use.
    let app_clone = App {};
    thread::spawn(move || {
        let input2 = vec![66u8; 10]; // input: 10 bytes of ASCII 'B'
        let _ = app_clone.process(&input2);
    })
    .join()
    .unwrap();
}