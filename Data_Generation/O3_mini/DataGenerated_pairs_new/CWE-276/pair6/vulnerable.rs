use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;

struct Service {
    file_path: String,
}

impl Service {
    fn new(path: &str) -> Self {
        Service {
            file_path: path.to_string(),
        }
    }

    fn execute(&self) -> io::Result<()> {
        let shared_path = Arc::new(self.file_path.clone());
        let mut handles = Vec::new();

        for _ in 0..2 {
            let path_clone = Arc::clone(&shared_path);
            let handle = thread::spawn(move || -> io::Result<()> {
                // Insecure block: using unsafe to bypass normal checks and setting overly permissive mode
                unsafe {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .mode(0o777) // Overly permissive file permission (world-writable)
                        .open(&*path_clone)?;
                    writeln!(&file, "Sensitive Data")?;
                }
                Ok(())
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let service = Service::new("tempfile.txt");
    service.execute()
}