use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::str;
struct Data {
    ptr: *mut u8,
    size: usize,
}
impl Data {
    fn new(input: &str) -> Self {
        unsafe {
            let buf_size = std::mem::size_of_val(&input); 
            let layout = Layout::from_size_align(buf_size, 1).unwrap();
            let raw_ptr = alloc(layout);
            if raw_ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr::copy_nonoverlapping(input.as_ptr(), raw_ptr, buf_size);
            Data { ptr: raw_ptr, size: buf_size }
        }
    }
    fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.size))
        }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.size, 1).unwrap();
            dealloc(self.ptr, layout);
        }
    }
}
fn main() {
    let input = "This is a somewhat long input string causing miscalculation.";
    let stored = Data::new(input);
    println!("Stored content: '{}'", stored.as_str());
}
