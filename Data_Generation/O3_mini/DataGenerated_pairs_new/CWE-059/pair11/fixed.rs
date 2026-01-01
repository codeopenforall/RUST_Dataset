/////////////////////////////////////////////
// Corrected version
/////////////////////////////////////////////
use std::fs::{self, File};
use std::io::{self, Read};
use std::env;
use std::os::unix::fs::MetadataExt;
use std::thread;
use std::time::Duration;

fn process_file(path: &str) -> io::Result<String> {
    // Read metadata from the path.
    let initial = fs::metadata(path)?;
    if !initial.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "Not a file"));
    }

    // Open the file first; this is less vulnerable since we can examine the opened file.
    let mut file = File::open(path)?;

    // Immediately fetch metadata from the opened file descriptor.
    let current = file.metadata()?;
    // Validate that the inode and device match the metadata from the path.
    if initial.ino() != current.ino() || initial.dev() != current.dev() {
        return Err(io::Error::new(io::ErrorKind::Other, "File changed (TOCTOU detected)"));
    }

    // An artificial delay is still introduced, but now after verifying the file identity.
    thread::sleep(Duration::from_millis(50));

    // Proceed to read the file content safely.
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    match process_file(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => {
            eprintln!("Error processing file: {:?}", e);
            std::process::exit(1);
        }
    }
}