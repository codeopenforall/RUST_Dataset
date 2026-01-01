use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::env::current_exe;
struct FileProcessor;
impl FileProcessor {
    fn process(&self, path: &str) -> io::Result<String> {
        let file = File::open(path)?;
        let _metadata = file.metadata()?;
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
