////////////////////////////////////////////////////////////////
// Vulnerable Code Sample
////////////////////////////////////////////////////////////////
use std::env;
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Handler;

impl Handler {
    // This function first checks the file via metadata() and then, after a delay,
    // it reopens the file using the provided path.
    // An attacker may swap the symlink in the meantime, resulting in reading unexpected content.
    pub fn execute(path: &str) -> Result<String, std::io::Error> {
        // First, check the file by reading metadata.
        let meta = metadata(path)?;
        if !meta.is_file() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not a regular file"));
        }

        // Simulate a delay (e.g. other concurrent activity) that creates a TOCTOU window.
        thread::sleep(Duration::from_millis(100));

        // Concurrent unsafe activity to mimic realistic systems code.
        // We create a valid static array and unsafely read from it.
        let dummy_data = [0x41u8, 0x42, 0x43, 0x44];
        let safe_val: u8 = unsafe {
            // Unsafe raw pointer access.
            let ptr = dummy_data.as_ptr();
            *ptr.offset(2)
        };
        // This unsafe block is not directly related to the file vulnerability,
        // but mimics the use of unsafe code in complex systems.
        let _ = safe_val;

        // Now open the file using the original path.
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