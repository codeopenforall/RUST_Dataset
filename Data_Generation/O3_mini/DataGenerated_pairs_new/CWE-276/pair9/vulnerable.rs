///////////////////////////////////////////////////////////////
// Vulnerability Example: Insecure File Permission Setup
///////////////////////////////////////////////////////////////
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::thread;

struct Configurator {
    file_path: String,
}

impl Configurator {
    fn new(path: &str) -> Self {
        Self {
            file_path: path.to_owned(),
        }
    }

    // This method spawns a thread to create a configuration file.
    // It uses an unsafe block to simulate low-level pointer work.
    fn setup(&self) {
        let path = self.file_path.clone();
        let handle = thread::spawn(move || {
            unsafe {
                // POTENTIAL VULNERABILITY: Creating file with overly permissive mode.
                // Using 0o777 gives read, write, and execute permissions to everyone.
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o777) // Insecure default permissions: CWE-276
                    .open(&path)
                    .expect("Error opening file");
                // Dummy unsafe pointer manipulation to mimic unsafe low-level operations.
                let raw = file.as_raw_fd();
                let _dummy = raw as *mut i32;
            }
        });
        handle.join().expect("Thread panicked");
    }
}

fn main() {
    let configurator = Configurator::new("config.txt");
    configurator.setup();
    println!("Setup complete!");
}