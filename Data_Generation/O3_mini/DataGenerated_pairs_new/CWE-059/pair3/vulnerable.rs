///////////////////////////////////////////////
// Vulnerable Code (TOCTOU Symlink Race) Example
///////////////////////////////////////////////
use std::env;
use std::fs;
use std::io::{self, Read};
use std::time::Duration;
use std::thread;

fn process_file(path: &str) -> io::Result<String> {
    // Line 9: Retrieve metadata using fs::metadata (follows symlinks)
    let _meta = fs::metadata(path)?;
    // Introduce a delay simulating a TOCTOU window
    thread::sleep(Duration::from_millis(50));
    // Line 12: Open the file (if a symlink was swapped in during the delay, it is followed)
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    // Use an unsafe conversion to String (simulating usage of unsafe constructs)
    let content = unsafe { String::from_utf8_unchecked(buffer) };
    Ok(content)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "testfile.txt" };
    match process_file(path) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error processing file: {}", e),
    }
}