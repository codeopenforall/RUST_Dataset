#![allow(unused)]
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::mem;
struct Processor;
impl Processor {
    unsafe fn process(&self, input: &str) -> *mut u8 {
        let buf_size = mem::size_of_val(&input);
        let layout = Layout::from_size_align(buf_size, mem::align_of::<u8>()).unwrap();
        let buf = alloc(layout);
        if buf.is_null() {
            panic!("Allocation failed");
        }
        ptr::copy_nonoverlapping(input.as_ptr(), buf, buf_size);
        buf
    }
}
fn main() {
    let handler = Processor;
    let text = "Hello, Rust world!"; 
    unsafe {
        let raw = handler.process(text);
        let slice = std::slice::from_raw_parts(raw, mem::size_of_val(&text));
        println!("Copied buffer: {:?}", slice);
        let layout = Layout::from_size_align(mem::size_of_val(&text), mem::align_of::<u8>()).unwrap();
        dealloc(raw, layout);
    }
}
