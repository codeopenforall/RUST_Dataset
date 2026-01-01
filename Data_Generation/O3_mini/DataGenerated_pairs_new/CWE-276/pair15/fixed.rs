//////////////////////////////
// Corrected Code Example   //
// Fixed: Enforce restrictive file permissions (0o600) to mitigate CWE-276  //
//////////////////////////////
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Handler {
    log_path: String,
}

impl Handler {
    // Adjusted function to set secure file mode.
    unsafe fn initialize_log(&self) -> io::Result<()> {
        // Use a secure permission mode: 0o600 (read/write for owner only)
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600) // Fixed: Restrictive file permissions.
            .open(&self.log_path)?;
        // Simulate low-level file operation with unsafe pointer.
        let ptr = Box::into_raw(Box::new(file));
        (*ptr).write_all(b"Handler started\n")?;
        Box::from_raw(ptr);
        Ok(())
    }

    fn run(&self) -> io::Result<()> {
        let arc_path = Arc::new(self.log_path.clone());
        let path_clone = arc_path.clone();
        let thread_handle = thread::spawn(move || {
            // Mimic background processing delay.
            thread::sleep(Duration::from_millis(100));
            let _ = OpenOptions::new()
                .append(true)
                .open(&*path_clone)
                .and_then(|mut f| f.write_all(b"Background thread log\n"));
        });

        unsafe {
            self.initialize_log()?;
        }
        thread_handle.join().unwrap();
        Ok(())
    }
}

fn main() {
    let h = Handler {
        log_path: "tempfile.txt".to_string(),
    };
    if let Err(e) = h.run() {
        eprintln!("Error during processing: {}", e);
    }
}