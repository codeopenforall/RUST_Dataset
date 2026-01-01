use std::fs;
use std::thread;
use std::sync::Arc;
use std::io;
struct Processor;
impl Processor {
    fn run(&self, data: &str) -> io::Result<()> {
        let len = data.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.as_mut_ptr(), len);
        }
        let message = format!("Processed: {}", String::from_utf8_lossy(&buffer));
        let msg_arc = Arc::new(message);
        let mut handles = Vec::new();
        for _ in 0..2 {
            let msg_clone = Arc::clone(&msg_arc);
            let handle = thread::spawn(move || -> io::Result<()> {
                unsafe {
                    Self::update_file(&msg_clone)?
                }
                Ok(())
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join()
                  .map_err(|_| io::Error::new(io::ErrorKind::Other, "thread join failed"))??
        }
        Ok(())
    }
    unsafe fn update_file(msg: &str) -> io::Result<()> {
        if msg.contains("fail") {
            return Err(io::Error::new(io::ErrorKind::Other, "simulated failure"));
        }
        fs::write("log.txt", msg)
    }
}
fn main() {
    let processor = Processor;
    let input = "test input";
    if let Err(e) = processor.run(input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
