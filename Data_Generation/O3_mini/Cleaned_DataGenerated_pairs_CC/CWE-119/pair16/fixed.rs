struct MemoryBuffer {
    data: Vec<u32>,
}
impl MemoryBuffer {
    fn new(size: usize) -> Self {
        Self { data: vec![0; size] }
    }
    pub fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            return Err("index out of bounds");
        }
        self.data[idx] = val;
        Ok(())
    }
}
fn run() -> Result<(), &'static str> {
    let mut buf = MemoryBuffer::new(10);
    buf.update(10, 100)?;
    Ok(())
}
fn main() {
    match run() {
        Ok(_) => println!("Operation succeeded"),
        Err(e) => println!("Error: {}", e),
    }
}
