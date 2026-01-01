use std::fs;
use std::thread;
use std::sync::Arc;
use std::io;
struct Processor;
impl Processor {
    fn run(&self, data: &str) -> bool {
        let len = data.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.as_mut_ptr(), len);
        }
        let message = format!("Processed: {}", String::from_utf8_lossy(&buffer));
        let msg_arc = Arc::new(message);
        let handles: Vec<_> = (0..2).map(|_| {
            let msg_clone = Arc::clone(&msg_arc);
            thread::spawn(move || {
                unsafe {
                    let _ = Self::update_file(&msg_clone);
                }
            })
        }).collect();
        for handle in handles {
            let _ = handle.join();
        }
        true
    }
    unsafe fn update_file(msg: &str) -> io::Result<()> {
        if msg.contains("fail") {
            return Err(io::Error::new(io::ErrorKind::Other, "simulated failure"));
        }
        let _ = fs::write("log.txt", msg);
        Ok(())
    }
}
fn main() {
    let processor = Processor;
    let input = "test input";
    processor.run(input);
}
