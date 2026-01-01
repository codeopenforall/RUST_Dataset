use std::env;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Handler;
impl Handler {
    pub fn execute(path: &str) -> Result<String, std::io::Error> {
        let meta = metadata(path)?;
        if !meta.is_file() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not a regular file"));
        }
        thread::sleep(Duration::from_millis(100));
        let dummy_data = [0x41u8, 0x42, 0x43, 0x44];
        let safe_val: u8 = unsafe {
            let ptr = dummy_data.as_ptr();
            *ptr.offset(2)
        };
        let _ = safe_val;
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    match Handler::execute(&args[1]) {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
