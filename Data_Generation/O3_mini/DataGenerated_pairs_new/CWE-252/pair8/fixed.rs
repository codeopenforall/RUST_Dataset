use std::fs;
use std::thread;
use std::sync::Arc;
use std::io;

struct Processor;

impl Processor {
    // Processes the provided data and returns a Result indicating success or failure.
    fn run(&self, data: &str) -> io::Result<()> {
        // Unsafe memory manipulation: copy the input data into a new buffer.
        let len = data.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.as_mut_ptr(), len);
        }
        
        let message = format!("Processed: {}", String::from_utf8_lossy(&buffer));
        let msg_arc = Arc::new(message);
        
        let mut handles = Vec::new();
        // Spawn two threads to concurrently update a file with the message.
        // Each thread now propagates any I/O error from updating the file.
        for _ in 0..2 {
            let msg_clone = Arc::clone(&msg_arc);
            let handle = thread::spawn(move || -> io::Result<()> {
                unsafe {
                    // Check and propagate the result of update_file.
                    Self::update_file(&msg_clone)?
                }
                Ok(())
            });
            handles.push(handle);
        }
        
        // Join all threads, propagating any errors from the threads.
        for handle in handles {
            handle.join()
                  .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread join failed"))??
        }
        
        Ok(())
    }
    
    // Unsafe function that writes the provided message to a log file.
    // Returns an error when the message contains "fail".
    unsafe fn update_file(msg: &str) -> io::Result<()> {
        if msg.contains("fail") {
            return Err(io::Error::new(io::ErrorKind::Other, "simulated failure"));
        }
        // Propagate any potential error from the file write.
        fs::write("log.txt", msg)
    }
}

fn main() {
    let processor = Processor;
    let input = "test input";
    // If run() returns an Err, print the error and exit with a non-zero code.
    if let Err(e) = processor.run(input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}