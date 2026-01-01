/////////////////////////////////////////
// Secure Implementation (TOCTOU Fixed) //
/////////////////////////////////////////
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::env::current_exe;

struct FileProcessor;

impl FileProcessor {
    // In this secure version, the file is opened immediately, and its metadata is
    // obtained from the file handle. This approach ensures that the file being read is the same
    // one that was opened, eliminating the symlink swapping race condition.
    fn process(&self, path: &str) -> io::Result<String> {
        // Open the file directly without performing a separate metadata check.
        let file = File::open(path)?;

        // Retrieve metadata from the file handle to prevent TOCTOU issues.
        let _metadata = file.metadata()?;

        // Read the file content directly from the file handle.
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
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