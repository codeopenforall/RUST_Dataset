use std::alloc::{alloc, Layout};
use std::ptr;
struct Processor;
impl Processor {
    pub fn process(input: &str) -> String {
        unsafe {
            let correct_size = input.len();
            let layout = Layout::from_size_align(correct_size, 1).unwrap();
            let buf = alloc(layout);
            if buf.is_null() {
                panic!("Allocation failed");
            }
            ptr::copy_nonoverlapping(input.as_ptr(), buf, correct_size);
            String::from_raw_parts(buf, correct_size, correct_size)
        }
    }
}
fn main() {
    let data = "Hello, world!";
    let result = Processor::process(data);
    println!("{}", result);
}
