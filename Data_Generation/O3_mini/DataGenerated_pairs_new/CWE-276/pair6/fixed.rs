use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;

struct Module {
    file_path: String,
}

impl Module {
    fn new(path: &str) -> Self {
        Module {
            file_path: path.to_string(),
        }
    }

    fn run(&self) -> io::Result<()> {
        let shared_path = Arc::new(self.file_path.clone());
        let mut handles = Vec::new();

        for _ in 0..2 {
            let path_clone = Arc::clone(&shared_path);
            let handle = thread::spawn(move || -> io::Result<()> {
                // Secure file creation: setting restrictive permissions (owner read/write only, mode 0o600)
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600)
                    .open(&*path_clone)?;
                writeln!(&file, "Sensitive Data")?;
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
    let module = Module::new("tempfile.txt");
    module.run()
}