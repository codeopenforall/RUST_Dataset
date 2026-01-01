#!/usr/bin/env rust
// This is a realistic simulation of a buffer copy vulnerability
// in a multithreaded context using unsafe Rust.
// The code defines a common application struct and uses an unsafe block
// to copy data from a source slice to an allocated vector.
// The bug arises because the length passed to the copy is miscalculated,
// allowing the unsafe copy to write past the allocated memory.
//
// Compile with: rustc vulnerable.rs

use std::ptr;
use std::thread;

pub struct App {}

impl App {
    // Performs an unsafe copy of data.
    // It deliberately miscalculates the length to copy the source slice length + 2,
    // which may exceed the allocated destination capacity.
    pub fn process(&self, data: &[u8]) -> Vec<u8> {
        // Allocate buffer with fixed capacity.
        let mut buf: Vec<u8> = Vec::with_capacity(10);
        // Mis-calculate the length to copy (violating API preconditions).
        let copy_len = data.len() + 2;
        unsafe {
            // Vulnerability: Using copy_nonoverlapping without ensuring that the destination
            // has enough capacity. This can copy beyond the allocated memory.
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), copy_len);
            // Setting length to a value that exceeds the actual allocated capacity.
            buf.set_len(copy_len);
        }
        buf
    }
}

fn main() {
    let app = App {};
    let input = vec![65u8; 10]; // input: 10 bytes of ASCII 'A'
    let output = app.process(&input);
    println!("Output: {:?}", output);

    // Additionally, simulate a concurrent execution scenario.
    let app_clone = App {};
    thread::spawn(move || {
        let input2 = vec![66u8; 10]; // input: 10 bytes of ASCII 'B'
        let _ = app_clone.process(&input2);
    })
    .join()
    .unwrap();
}