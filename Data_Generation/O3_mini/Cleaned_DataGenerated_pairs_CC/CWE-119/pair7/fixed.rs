use std::ptr;
struct Data {
    buffer: [u8; 64],
    magic: u32,
}
impl Data {
    pub fn new() -> Self {
        Self {
            buffer: [0; 64],
            magic: 0xDEADBEEF,
        }
    }
    pub fn operate(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.buffer.len() {
            return Err("Input size exceeds buffer capacity");
        }
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            for i in 0..input.len() {
                *dest.add(i) = input[i];
            }
        }
        Ok(())
    }
    pub fn check(&self) -> bool {
        self.magic == 0xDEADBEEF
    }
}
fn main() {
    let mut obj = Data::new();
    let input = vec![1u8; 100];
    let res = obj.operate(&input);
    assert!(res.is_err(), "Operation should fail for oversized input");
    println!("Operation rejected oversized input safely");
}
