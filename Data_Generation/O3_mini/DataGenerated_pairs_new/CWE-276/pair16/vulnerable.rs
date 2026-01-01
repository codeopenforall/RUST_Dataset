//
// Insecure file permission handling using overly permissive mode (0o777)
// CWE-276: Incorrect Default Permissions (Overly Permissive Settings)
// This code mimics a real-world vulnerability where a file is created with insecure permissions.
// It uses unsafe blocks, concurrency, and smart pointers to simulate a complex system.
//
use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Write, Read};

struct Manager {
    path: String,
    data: Arc<Mutex<String>>,
}

impl Manager {
    fn new(path: &str) -> Manager {
        Manager {
            path: path.to_string(),
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    // Unsafe file creation with overly permissive mode (0o777)
    unsafe fn create_file(&self) -> std::io::Result<File> {
        // FLAW: The file is created with 0o777, which can grant excessive permissions to other users.
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o777)  // Vulnerable line: insecure default permissions.
            .open(&self.path)?;
        Ok(file)
    }

    fn write_data(&self, content: &str) -> std::io::Result<()> {
        // The unsafe block is used to call the unsafe file creation method.
        let file = unsafe { self.create_file()? };
        unsafe {
            let mut file = file;
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }

    fn read_data(&self) -> std::io::Result<String> {
        let mut file = File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

fn main() {
    // This file path is used for demonstration and testing purposes.
    let manager = Manager::new("/tmp/test_insecure_file.txt");
    let manager_ref = Arc::new(manager);
    let mut handles = vec![];

    // Spawn multiple threads to simulate concurrent file writes.
    for _ in 0..3 {
        let manager_clone = Arc::clone(&manager_ref);
        let handle = thread::spawn(move || {
            let data = "Sensitive Data";
            let _ = manager_clone.write_data(data);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish execution.
    for handle in handles {
        handle.join().unwrap();
    }

    // Read and print the file content.
    let content = manager_ref.read_data().unwrap();
    println!("File content: {}", content);
}