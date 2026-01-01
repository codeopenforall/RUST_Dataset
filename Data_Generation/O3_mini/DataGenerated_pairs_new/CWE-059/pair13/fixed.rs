////////////////////////////////////////////////////////////////
// Fixed Code Sample
////////////////////////////////////////////////////////////////
use std::env;
use std::fs::{canonicalize, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Handler;

impl Handler {
    // This function resolves the true (canonical) path of the file first.
    // Even if the original path is a symlink and is later swapped,
    // the canonical path remains pointing to the initially intended file.
    pub fn execute(path: &str) -> Result<String, std::io::Error> {
        // Resolve the canonical, absolute path.
        let safe_path = canonicalize(path)?;
        // Double-check that the resolved path points to a regular file.
        let meta = std::fs::metadata(&safe_path)?;
        if !meta.is_file() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not a regular file"));
        }

        // Simulate similar delay as in the vulnerable version.
        thread::sleep(Duration::from_millis(100));

        // Mimic concurrent unsafe operation.
        let dummy_data = [0x41u8, 0x42, 0x43, 0x44];
        let safe_val: u8 = unsafe {
            let ptr = dummy_data.as_ptr();
            *ptr.offset(2)
        };
        let _ = safe_val;

        // Open the file using the safely resolved canonical path.
        let mut file = File::open(&safe_path)?;
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