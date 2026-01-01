use std::fs::File;
use std::io::{self, Read};
use std::path::{PathBuf};
use std::sync::Arc;
use std::thread;

struct Loader {
    base: PathBuf,
}

impl Loader {
    fn new(base: &str) -> Self {
        Loader { base: PathBuf::from(base) }
    }

    // Reads a file by joining a user-supplied relative path to a fixed base directory.
    // This implementation is vulnerable to path traversal as it blindly joins the path.
    // It further uses an unsafe block within a concurrent context to mimic lower-level file operations.
    fn load_file(&self, relative: &str) -> io::Result<String> {
        // Vulnerability: No canonicalization or proper validation is performed.
        let full_path = self.base.join(relative);
        let file = File::open(&full_path)?;
        let metadata = file.metadata()?;
        let size = metadata.len() as usize;
        let mut buffer = Vec::with_capacity(size);

        let shared_file = Arc::new(file);
        let mut handles = vec![];
        let part = if size >= 2 { size / 2 } else { size }; // Handle small files

        // Simulating concurrent partial file processing using threads.
        for i in 0..2 {
            let _file_clone = Arc::clone(&shared_file);
            let start = i * part;
            let end = if i == 1 { size } else { (i + 1) * part };
            let mut local_buf = vec![0u8; end - start];
            let handle = thread::spawn(move || {
                unsafe {
                    // Unsafe block used to simulate low-level memory manipulation.
                    let ptr = local_buf.as_mut_ptr();
                    for j in 0..(end - start) {
                        *ptr.add(j) = 0; // In a real scenario, this would be reading bytes.
                    }
                }
                (start, local_buf)
            });
            handles.push(handle);
        }
        for handle in handles {
            let (_start, local_buf) = handle.join().unwrap();
            buffer.extend(local_buf);
        }
        let content = String::from_utf8_lossy(&buffer).to_string();
        Ok(content)
    }
}

fn main() {
    let loader = Loader::new("./safe_dir");
    let args: Vec<String> = std::env::args().collect();
    let target = if args.len() > 1 { &args[1] } else { "example.txt" };
    match loader.load_file(target) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}