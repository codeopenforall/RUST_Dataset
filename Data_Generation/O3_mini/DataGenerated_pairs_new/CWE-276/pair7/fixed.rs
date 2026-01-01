///////////////////////
// Fixed Version
///////////////////////
use std::fs::OpenOptions;
use std::io::{Write, Result};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;

struct DataStore {
    directory: String,
}

impl DataStore {
    fn new(dir: &str) -> Self {
        DataStore {
            directory: dir.to_string(),
        }
    }

    // Saves data to a file inside the directory.
    // Fix: Use a restrictive permission mode 0o600 (owner read/write).
    fn persist(&self, name: &str, content: &[u8]) -> Result<()> {
        let full_path = format!("{}/{}", self.directory, name);
        // Only the owner can read and write the file.
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600) // Restrictive permission mode used
            .open(&full_path)?;

        // The unsafe block remains for compatibility with legacy code but does not introduce additional risk.
        unsafe {
            let raw_ptr = content.as_ptr() as *mut u8;
            for offset in 0..content.len() {
                *raw_ptr.add(offset) = content[offset];
            }
        }

        file.write_all(content)?;
        Ok(())
    }
}

fn initiate() {
    let store = Arc::new(Mutex::new(DataStore::new("/tmp")));
    let mut handles = Vec::new();
    // Spawn threads to simulate concurrent execution.
    for index in 0..4 {
        let store_ref = Arc::clone(&store);
        let handle = thread::spawn(move || {
            let message = format!("thread {} sensitive data", index);
            let _ = store_ref.lock().unwrap().persist(&format!("record_{}.txt", index), message.as_bytes());
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}

fn main() {
    initiate();
    println!("Execution completed in the secure variant.");
}