use std::sync::{Arc, Mutex};
struct MemoryHandler {
    data: Box<[u8; 10]>,
}
impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler { data: Box::new([0; 10]) }
    }
    pub fn update(&mut self, index: usize, value: u8) {
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
        }
    }
    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&x| x as u32).sum()
    }
}
fn process() -> Result<u32, &'static str> {
    let mut handler = MemoryHandler::new();
    handler.update(10, 42);
    Ok(handler.sum())
}
fn main() {
    match process() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
