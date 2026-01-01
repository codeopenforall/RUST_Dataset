//////////////////////////////
// Vulnerable Code Example  //
// CWE-276: Incorrect Default Permissions - Overly permissive file mode  //
//////////////////////////////
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Server {
    log_path: String,
}

impl Server {
    // Unsafe raw pointer access to simulate low-level file handling in concurrency.
    unsafe fn create_log(&self) -> io::Result<()> {
        // Introduce vulnerability: overly permissive file mode (0o777)
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o777) // Vulnerable line: overly permissive file permissions.
            .open(&self.log_path)?;
        // Use unsafe block to simulate raw pointer manipulation.
        let ptr = Box::into_raw(Box::new(file));
        // Write a log entry in a simulated concurrent environment.
        (*ptr).write_all(b"Server started\n")?;
        // Convert raw pointer back into a Box and drop to close file.
        Box::from_raw(ptr);
        Ok(())
    }

    fn run(&self) -> io::Result<()> {
        // Spawn a thread that writes logs concurrently.
        let arc_self = Arc::new(self.log_path.clone());
        let log_clone = arc_self.clone();
        let handle = thread::spawn(move || {
            // Artificial delay to simulate async behavior.
            thread::sleep(Duration::from_millis(100));
            // Directly open and append logs.
            let _ = OpenOptions::new()
                .append(true)
                .open(&*log_clone)
                .and_then(|mut f| f.write_all(b"Background thread log\n"));
        });

        // Main thread creates the log file.
        unsafe {
            self.create_log()?;
        }
        handle.join().unwrap();
        Ok(())
    }
}

fn main() {
    let srv = Server {
        log_path: "tempfile.txt".to_string(),
    };
    if let Err(e) = srv.run() {
        eprintln!("Error running server: {}", e);
    }
}