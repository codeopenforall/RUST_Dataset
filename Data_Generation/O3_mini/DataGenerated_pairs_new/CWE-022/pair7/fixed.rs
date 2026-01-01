use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;

struct Loader {
    base: PathBuf,
}

impl Loader {
    fn new(base: &str) -> Self {
        // Canonicalize the base directory to ensure it has a proper absolute path.
        let canonical_base = std::fs::canonicalize(base).expect("Invalid base directory");
        Loader { base: canonical_base }
    }

    // Validates that the candidate path is within the allowed base directory.
    fn is_allowed(&self, candidate: &Path) -> bool {
        if let Ok(canonical_candidate) = candidate.canonicalize() {
            canonical_candidate.starts_with(&self.base)
        } else {
            false
        }
    }

    // Reads file content after ensuring the resulting path remains within the allowed directory.
    fn load_file(&self, relative: &str) -> io::Result<String> {
        let full_path = self.base.join(relative);
        // Check if the canonicalized path is within the allowed base directory.
        if !self.is_allowed(&full_path) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Path traversal detected",
            ));
        }
        let file = File::open(&full_path)?;
        let metadata = file.metadata()?;
        let size = metadata.len() as usize;
        let mut buffer = Vec::with_capacity(size);

        let shared_file = Arc::new(file);
        let mut handles = vec![];
        let part = if size >= 2 { size / 2 } else { size };

        for i in 0..2 {
            let _file_clone = Arc::clone(&shared_file);
            let start = i * part;
            let end = if i == 1 { size } else { (i + 1) * part };
            let mut local_buf = vec![0u8; end - start];
            let handle = thread::spawn(move || {
                unsafe {
                    let ptr = local_buf.as_mut_ptr();
                    for j in 0..(end - start) {
                        *ptr.add(j) = 0;
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