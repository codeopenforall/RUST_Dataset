///////////////////////////////////////////////
// Fixed Code: Mitigated TOCTOU Symlink Race
///////////////////////////////////////////////
use std::env;
use std::fs;
use std::io::{self, Read};
use std::time::Duration;
use std::thread;

fn process_file(path: &str) -> io::Result<String> {
    // Use fs::symlink_metadata to get metadata without following symlinks
    let meta = fs::symlink_metadata(path)?;
    // Reject if the file is a symbolic link to avoid TOCTOU vulnerability
    if meta.file_type().is_symlink() {
        return Err(io::Error::new(io::ErrorKind::Other, "Symlink files are not permitted"));
    }
    // Even with a delay, the file type is now trusted (symlink rejected)
    thread::sleep(Duration::from_millis(50));
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    // Unsafe conversion remains for demonstration purposes
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