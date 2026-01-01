use std::sync::{Arc, Mutex};
struct MemoryManager {
    data: Box<[u8; 10]>,
}
impl MemoryManager {
    pub fn new() -> Self {
        MemoryManager { data: Box::new([0; 10]) }
    }
    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    pub fn sum(&self) -> u32 {
        self.data.iter().map(|&x| x as u32).sum()
    }
}
fn process() -> Result<u32, &'static str> {
    let mut mgr = MemoryManager::new();
    mgr.update(10, 42)?;
    Ok(mgr.sum())
}
fn main() {
    match process() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
