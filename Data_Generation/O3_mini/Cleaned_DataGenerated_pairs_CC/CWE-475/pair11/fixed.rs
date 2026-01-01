#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Data {
    buf: *mut u32,
    len: usize,
}
impl Data {
    fn new(size: usize) -> Self {
        let mut v = vec![0u32; size];
        let boxed = v.into_boxed_slice();
        let ptr = Box::into_raw(boxed) as *mut u32;
        Data { buf: ptr, len: size }
    }
    fn write(&self, index: usize, value: u32) {
        if index >= self.len {
            panic!("Index {} out of bounds for length {}", index, self.len);
        }
        unsafe {
            *self.buf.add(index) = value;
        }
    }
    fn sum(&self) -> u32 {
        let mut s = 0;
        unsafe {
            for i in 0..self.len {
                s += *self.buf.add(i);
            }
        }
        s
    }
    fn free(&self) {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.buf, self.len);
            let _ = Box::from_raw(slice);
        }
    }
}
pub fn run_harness() -> Result<u32, &'static str> {
    let data = Data::new(4);
    let result = std::panic::catch_unwind(|| {
        data.write(4, 42);
    });
    data.free();
    match result {
        Ok(_) => Ok(data.sum()),
        Err(_) => Err("Detected out-of-bounds write via panic"),
    }
}
fn main() {
    let res = run_harness();
    match res {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}
