#![allow(unused)]
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::mem;

/// A simple processor struct that provides a data copying method.
struct Processor;

impl Processor {
    // This method is intentionally flawed:
    // It calculates the buffer size using mem::size_of_val(&input) which returns the size of a str reference,
    // not the actual byte length of the string. On a 64‐bit system, this typically returns 16 bytes,
    // regardless of the length of the string content.
    unsafe fn process(&self, input: &str) -> *mut u8 {
        // FLAW: Miscalculate buffer size by using mem::size_of_val which yields the size of &str,
        // not the length of the string data.
        let buf_size = mem::size_of_val(&input);
        let layout = Layout::from_size_align(buf_size, mem::align_of::<u8>()).unwrap();
        let buf = alloc(layout);
        if buf.is_null() {
            panic!("Allocation failed");
        }
        // FLAW: Copies only buf_size bytes. For longer strings, this under-copies the data.
        ptr::copy_nonoverlapping(input.as_ptr(), buf, buf_size);
        buf
    }
}

fn main() {
    let handler = Processor;
    let text = "Hello, Rust world!"; // length != mem::size_of_val(&text) on most systems.
    unsafe {
        let raw = handler.process(text);
        // Reads exactly mem::size_of_val(&text) bytes from the allocated memory
        let slice = std::slice::from_raw_parts(raw, mem::size_of_val(&text));
        println!("Copied buffer: {:?}", slice);
        let layout = Layout::from_size_align(mem::size_of_val(&text), mem::align_of::<u8>()).unwrap();
        dealloc(raw, layout);
    }
}