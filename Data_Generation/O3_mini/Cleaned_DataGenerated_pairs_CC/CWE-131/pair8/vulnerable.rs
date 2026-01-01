use std::alloc::{alloc, Layout};
use std::ptr;
use std::mem;
struct Processor;
impl Processor {
    pub fn process(input: &str) -> String {
        unsafe {
            let wrong_size = mem::size_of_val(&input);
            let layout = Layout::from_size_align(wrong_size, 1).unwrap();
            let buf = alloc(layout);
            if buf.is_null() {
                panic!("Allocation failed");
            }
            ptr::copy_nonoverlapping(input.as_ptr(), buf, wrong_size);
            String::from_raw_parts(buf, wrong_size, wrong_size)
        }
    }
}
fn main() {
    let data = "Hello, world!";
    let result = Processor::process(data);
    println!("{}", result);
}
