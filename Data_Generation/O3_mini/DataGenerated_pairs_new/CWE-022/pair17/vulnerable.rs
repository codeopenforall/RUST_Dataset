use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

struct FileServer {
    base: PathBuf,
}

impl FileServer {
    fn new(base: &str) -> Self {
        Self {
            base: PathBuf::from(base),
        }
    }

    // This method naively joins the user-supplied path to the base
    // without validating that the result stays within the intended directory.
    fn process(&self, rel_path: &str, data: &str) -> std::io::Result<()> {
        // Vulnerability: using PathBuf::join without security checks allows path traversal.
        let target = self.base.join(rel_path);

        // Unsafe block simulating low-level operations in a concurrent context.
        unsafe {
            let raw = target.to_str().unwrap().as_ptr();
            // Dummy memory access simulation (for realistic unsafe usage)
            let _ = *raw;
        }

        // Write the data to the target file regardless of its location.
        let mut file = File::create(target)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

fn main() {
    // Create a shared FileServer using a Mutex for thread-safe access.
    let server = Arc::new(Mutex::new(FileServer::new("./data")));

    // Spawn a thread simulating a concurrent file operation.
    let srv_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        let server_lock = srv_clone.lock().unwrap();
        // ATTACKER-CONTROLLED INPUT: Passing a path with "../" escapes the intended directory.
        let _ = server_lock.process("../outside.txt", "malicious data");
    });

    handle.join().unwrap();
    println!("Main operation complete (vulnerable version).");
}