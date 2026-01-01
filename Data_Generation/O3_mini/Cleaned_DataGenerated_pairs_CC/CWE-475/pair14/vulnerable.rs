use std::panic;
struct MemoryBlock {
    buffer: Box<[i32]>,
}
impl MemoryBlock {
    fn new() -> Self {
        Self {
            buffer: vec![10, 20, 30, 40, 50].into_boxed_slice(),
        }
    }
    fn dangerous_sum(&self, offset: usize, count: usize) -> i32 {
        unsafe {
            let ptr = self.buffer.as_ptr().add(offset);
            let slice = std::slice::from_raw_parts(ptr, count);
            slice.iter().sum()
        }
    }
    fn safe_entry(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        let res = panic::catch_unwind(|| self.dangerous_sum(offset, count));
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err("Operation panicked"),
        }
    }
}
fn main() {
    let block = MemoryBlock::new();
    let valid = block.safe_entry(1, 3);
    println!("Valid call result: {:?}", valid);
    let invalid = block.safe_entry(2, 4);
    println!("Invalid call result: {:?}", invalid);
}
