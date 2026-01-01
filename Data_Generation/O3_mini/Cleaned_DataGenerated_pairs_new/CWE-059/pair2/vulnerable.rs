use std::env;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
struct FileProcessor;
impl FileProcessor {
    fn process(&self, path: &str) -> io::Result<String> {
        let metadata = fs::metadata(path)?;
        unsafe {
            let dummy_ptr: *const u8 = &metadata as *const _ as *const u8;
            let _ = *dummy_ptr; 
        }
        thread::sleep(Duration::from_millis(100));
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let processor = FileProcessor;
    match processor.process(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error processing file: {}", e),
    }
}
