use std::sync::{Arc, Mutex};
use std::thread;
struct Computor {
    value: u64,
}
impl Computor {
    fn new(val: u64) -> Self {
        Computor { value: val }
    }
    unsafe fn subtract(&mut self, sub: u64) {
        let ptr = &mut self.value as *mut u64;
        *ptr = *ptr - sub; 
    }
    fn get(&self) -> u64 {
        self.value
    }
}
pub fn process(input: u64, sub: u64) -> Result<u64, &'static str> {
    let mut comp = Computor::new(input);
    unsafe {
        comp.subtract(sub);
    }
    Ok(comp.get())
}
fn main() {
    match process(100, 10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
