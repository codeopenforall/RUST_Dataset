use std::thread;
struct BufferHandler {
    buffer: Vec<u32>,
}
impl BufferHandler {
    fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size + 1); }
        for i in 0..size {
            buffer[i] = 0;
        }
        Self { buffer }
    }
    fn update(&mut self, index: usize, value: u32) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(index) = value;
        }
    }
    fn aggregate(&self) -> u32 {
        self.buffer.iter().sum()
    }
}
fn process() -> u32 {
    let mut handler = BufferHandler::new(10);
    let join_handle = thread::spawn(move || {
        handler.update(10, 42);
        handler.aggregate()
    });
    join_handle.join().unwrap()
}
fn main() {
    let outcome = process();
    println!("Result: {}", outcome);
}
