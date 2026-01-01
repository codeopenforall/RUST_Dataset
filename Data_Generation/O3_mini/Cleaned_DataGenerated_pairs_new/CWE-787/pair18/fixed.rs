use std::thread;
struct BufferHandler {
    buffer: Vec<u32>,
}
impl BufferHandler {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, 0);
        Self { buffer }
    }
    fn update(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    fn aggregate(&self) -> u32 {
        self.buffer.iter().sum()
    }
}
fn process() -> u32 {
    let mut handler = BufferHandler::new(10);
    let join_handle = thread::spawn(move || {
        let _ = handler.update(10, 42);
        handler.aggregate()
    });
    join_handle.join().unwrap()
}
fn main() {
    let outcome = process();
    println!("Result: {}", outcome);
}
