use std::fs;
use std::thread;
use std::sync::Arc;
use std::io;

struct Processor;

impl Processor {
    // Processes the given data and returns a boolean status.
    // In this version, any I/O error from updating the file is silently ignored.
    fn run(&self, data: &str) -> bool {
        // Unsafe memory manipulation: copy the input data into a new buffer.
        let len = data.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.as_mut_ptr(), len);
        }
        
        let message = format!("Processed: {}", String::from_utf8_lossy(&buffer));
        let msg_arc = Arc::new(message);
        
        // Spawn two threads to concurrently update a file with the message.
        let handles: Vec<_> = (0..2).map(|_| {
            let msg_clone = Arc::clone(&msg_arc);
            thread::spawn(move || {
                unsafe {
                    // Vulnerability: ignoring the return value of update_file.
                    // If the message contains "fail", update_file returns an error,
                    // but this error is dropped.
                    let _ = Self::update_file(&msg_clone);
                }
            })
        }).collect();
        
        for handle in handles {
            let _ = handle.join();
        }
        
        true
    }
    
    // Unsafe function that attempts to write the provided message to a file.
    // If the message contains "fail", it returns an error.
    unsafe fn update_file(msg: &str) -> io::Result<()> {
        if msg.contains("fail") {
            return Err(io::Error::new(io::ErrorKind::Other, "simulated failure"));
        }
        // The result of the file write is intentionally ignored.
        let _ = fs::write("log.txt", msg);
        Ok(())
    }
}

fn main() {
    let processor = Processor;
    // Here, we use a constant input. In practice, this could come from elsewhere.
    let input = "test input";
    processor.run(input);
}