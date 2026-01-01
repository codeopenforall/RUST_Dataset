#![allow(unused)]
use std::slice;
struct Container {
    ptr: *const u32,
    len: usize,
}
impl Container {
    fn new(input: &[u32]) -> Self {
        Container {
            ptr: input.as_ptr(),
            len: input.len(),
        }
    }
    fn aggregate(&self) -> u32 {
        unsafe {
            let data = slice::from_raw_parts(self.ptr, self.len);
            data.iter().fold(0, |acc, &val| acc + val)
        }
    }
}
fn main() {
    let boxed = Box::new([1, 2, 3, 4, 42]);
    let valid_slice = &boxed[..4];
    let handler = Container::new(valid_slice);
    let result = handler.aggregate();
    println!("Total: {}", result);
    if result != 10 {
        println!("Error: Incorrect sum computed");
        std::process::exit(1);
    }
}
