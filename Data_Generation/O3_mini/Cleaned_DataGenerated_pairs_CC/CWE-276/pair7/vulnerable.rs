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
    fn persist(&self, name: &str, content: &[u8]) -> Result<()> {
        let full_path = format!("{}/{}", self.directory, name);
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o777) 
            .open(&full_path)?;
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
    println!("Execution completed in the insecure variant.");
}
