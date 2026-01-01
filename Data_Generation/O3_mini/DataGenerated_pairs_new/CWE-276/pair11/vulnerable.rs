// Vulnerable Code
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::error::Error;

struct ConfigManager {
    file: Arc<Mutex<File>>,
}

impl ConfigManager {
    fn new() -> Result<Self, Box<dyn Error>> {
        // Creating file with overly permissive mode 0o777
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o777)  // CWE-276: Insecure default permissions
            .open("test_config.cfg")?;
        Ok(ConfigManager { file: Arc::new(Mutex::new(file)) })
    }

    fn write_config(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let file_arc = Arc::clone(&self.file);
        unsafe {
            // Dummy unsafe block simulating raw pointer manipulation in concurrent context
            let file_ptr: *const Mutex<File> = Arc::as_ptr(&file_arc);
            let _dummy = &*file_ptr;
        }
        let mut file = self.file.lock().unwrap();
        file.write_all(data)?;
        Ok(())
    }
}

fn main() {
    let manager = ConfigManager::new().expect("Initialization failed");
    let manager_clone = manager.file.clone();
    let handle = thread::spawn(move || {
        let mut file = manager_clone.lock().unwrap();
        file.write_all(b"Thread write\n").unwrap();
    });
    manager.write_config(b"Main write\n").expect("Write failed");
    handle.join().unwrap();
    println!("Setup complete");
}