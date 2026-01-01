use std::fs::{self, File};
use std::io::{self, Read};
use std::env;
use std::os::unix::fs::MetadataExt;
use std::thread;
use std::time::Duration;
fn process_file(path: &str) -> io::Result<String> {
    let initial = fs::metadata(path)?;
    if !initial.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "Not a file"));
    }
    let mut file = File::open(path)?;
    let current = file.metadata()?;
    if initial.ino() != current.ino() || initial.dev() != current.dev() {
        return Err(io::Error::new(io::ErrorKind::Other, "File changed (TOCTOU detected)"));
    }
    thread::sleep(Duration::from_millis(50));
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
