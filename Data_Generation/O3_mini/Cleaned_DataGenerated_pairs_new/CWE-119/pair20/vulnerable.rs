use std::ptr;
struct BufferProcessor {
    buffer: [u8; 10],
}
impl BufferProcessor {
    fn new() -> Self {
        BufferProcessor { buffer: [0; 10] }
    }
    fn update(&mut self, input: &[u8]) -> Result<(), &'static str> {
        unsafe {
            let dest = self.buffer.as_mut_ptr();
            for i in 0..input.len() {
                ptr::write(dest.add(i), input[i]);
            }
        }
        Ok(())
    }
    fn read(&self) -> &[u8] {
        &self.buffer
    }
}
fn main() {
    let mut processor = BufferProcessor::new();
    let data = b"ABCDEFGHIJKL"; 
    match processor.update(data) {
        Ok(()) => println!("Content: {:?}", processor.read()),
        Err(e) => println!("Error: {}", e),
    }
}
