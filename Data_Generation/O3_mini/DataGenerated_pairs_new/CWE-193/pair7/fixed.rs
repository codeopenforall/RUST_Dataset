use std::sync::Arc;
use std::thread;

trait Summable {
    fn calculate(&self) -> u32;
}

struct BufferHandler {
    buf: Vec<u8>,
}

impl BufferHandler {
    fn new() -> Self {
        BufferHandler { buf: Vec::new() }
    }

    fn load(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }
}

impl Summable for BufferHandler {
    fn calculate(&self) -> u32 {
        let len = self.buf.len();
        let ptr = self.buf.as_ptr();
        // Correct iteration: only traverse indices 0..len.
        unsafe {
            let mut total: u32 = 0;
            for i in 0..len {
                total += *ptr.add(i) as u32;
            }
            total
        }
    }
}

fn main() {
    let mut handler = BufferHandler::new();
    // Known input: [1, 2, 3, 4] should sum to 10.
    handler.load(&[1, 2, 3, 4]);

    // Demonstrate usage with concurrency.
    let shared_handler = Arc::new(handler);
    let handler_clone = Arc::clone(&shared_handler);
    let worker = thread::spawn(move || {
        // Fixed calculation avoiding off-by-one error.
        handler_clone.calculate()
    });

    let result = worker.join().expect("Worker thread panicked");
    println!("Computed sum: {}", result);
}