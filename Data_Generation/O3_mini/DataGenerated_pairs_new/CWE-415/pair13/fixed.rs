#![allow(unused)]
struct Data {
    value: i32,
}

impl Data {
    fn new(v: i32) -> Self {
        Data { value: v }
    }
}

pub fn run() -> i32 {
    // Allocate a resource on the heap.
    let resource = Box::new(Data::new(10));
    // Convert it into a raw pointer.
    let raw_ptr = Box::into_raw(resource);
    unsafe {
        // Correctly reconstruct the Box only once.
        let box_a = Box::from_raw(raw_ptr);
        // Use the resource safely.
        box_a.value * 2
    }
}

fn main() {
    // Running fixed code.
    println!("Result: {}", run());
}