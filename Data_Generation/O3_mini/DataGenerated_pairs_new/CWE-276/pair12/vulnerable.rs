/* 
   This code simulates a system that creates, writes to, and operates on a file.
   It uses unsafe blocks, concurrency with threads and a mutex, and leverages
   Unix-specific file mode settings via OpenOptionsExt.
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

    // This unsafe method opens the file with overly permissive access mode 0o777.
    pub unsafe fn open_file(&self) -> std::io::Result<File> {
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            // Vulnerability: using insecure permission mode 0o777.
            .mode(0o777)
            .open(&self.path)
    }

    // Writes data concurrently to the file using a thread. Also uses insecure mode.
    pub fn write_data(&self, data: &str) -> std::io::Result<()> {
        let guard = Arc::new(Mutex::new(()));
        let file_path = self.path.clone();
        let guard_clone = guard.clone();
        let data_string = data.to_owned();
        let handle = thread::spawn(move || {
            let _lock = guard_clone.lock().unwrap();
            // Vulnerability: again using overly permissive 0o777 mode when opening.
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .mode(0o777)
                .open(&file_path)
                .expect("failed to open file");
            file.write_all(data_string.as_bytes()).expect("write failed");
        });
        handle.join().unwrap();
        Ok(())
    }

    // An arbitrary unsafe block simulating other low-level operations.
    pub fn unsafe_operation(&self) {
        unsafe {
            let mut value: i32 = 100;
            let ptr: *mut i32 = &mut value;
            *ptr += 10; // Unsafe pointer dereference.
            println!("Unsafe operation result: {}", value);
        }
    }
}

// Exposed run function for test purposes.
pub fn run(file_path: &str) {
    let handler = Handler::new(file_path);
    unsafe {
        handler.open_file().expect("Failed to open file unsafely");
    }
    handler.write_data("Vulnerable data write\n").expect("Write failed");
    handler.unsafe_operation();
}

fn main() {
    // Using a fixed file path.
    run("/tmp/insecure.txt");
}