/* 
   The revised code follows similar structure but restricts file permission modes to a secure default.
   It adjusts the file creation and appending operations to use permission mode 0o600,
   which grants read and write permissions only to the owner.
*/
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Handler {
    path: String,
}

impl Handler {
    pub fn new(path: &str) -> Self {
        Self { path: path.to_owned() }
    }

    // Securely open the file with restricted permission mode 0o600.
    pub unsafe fn open_file(&self) -> std::io::Result<File> {
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            // Fix: using secure permission mode 0o600.
            .mode(0o600)
            .open(&self.path)
    }

    // Writes data concurrently to the file using a thread.
    // The file is opened with the secure permission mode 0o600.
    pub fn write_data(&self, data: &str) -> std::io::Result<()> {
        let guard = Arc::new(Mutex::new(()));
        let file_path = self.path.clone();
        let guard_clone = guard.clone();
        let data_string = data.to_owned();
        let handle = thread::spawn(move || {
            let _lock = guard_clone.lock().unwrap();
            // Fix: using secure permission mode 0o600 when opening the file.
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .mode(0o600)
                .open(&file_path)
                .expect("failed to open file");
            file.write_all(data_string.as_bytes()).expect("write failed");
        });
        handle.join().unwrap();
        Ok(())
    }

    // Retains the same safe use of an unsafe block for simulation.
    pub fn unsafe_operation(&self) {
        unsafe {
            let mut value: i32 = 100;
            let ptr: *mut i32 = &mut value;
            *ptr += 10;
            println!("Unsafe operation result: {}", value);
        }
    }
}

// Exposed run function for testing purposes.
pub fn run(file_path: &str) {
    let handler = Handler::new(file_path);
    unsafe {
        handler.open_file().expect("Failed to open file securely");
    }
    handler.write_data("Secure data write\n").expect("Write failed");
    handler.unsafe_operation();
}

fn main() {
    // Using a fixed file path.
    run("/tmp/insecure.txt");
}