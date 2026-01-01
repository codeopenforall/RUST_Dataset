use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::slice;
use std::thread;
fn process(input: &str) -> Vec<u8> {
    unsafe {
        let size = input.len();
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        std::ptr::copy_nonoverlapping(input.as_ptr(), ptr, size);
        let result = slice::from_raw_parts(ptr, size).to_vec();
        dealloc(ptr, layout);
        result
    }
}
fn main() {
    let input = "This is a longer string that exceeds pointer size.";
    let handle = thread::spawn(move || {
        let res = process(input);
        println!("Output: {:?}", res);
    });
    handle.join().unwrap();
}
