use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;

struct Handler {
    file_path: String,
}

impl Handler {
    fn new(path: &str) -> Self {
        Handler {
            file_path: path.to_string(),
        }
    }

    fn run(&self) {
        // Using a mutex to synchronize concurrent file writes.
        let file_lock = Arc::new(Mutex::new(()));
        let mut threads = vec![];

        for _ in 0..4 {
            let file_lock = file_lock.clone();
            let path = self.file_path.clone();
            let thr = thread::spawn(move || {
                // An unsafe block performing a trivial pointer operation.
                unsafe {
                    let dummy: *mut u8 = &mut 0u8;
                    *dummy = 1;
                }

                // Lock critical section for file access.
                let _guard = file_lock.lock().unwrap();

                // Create the file with overly permissive permissions (0o777).
                let file_result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o777) // Vulnerability: Insecure file permission
                    .open(&path);

                if let Ok(mut file) = file_result {
                    use std::io::Write;
                    let _ = file.write_all(b"Entry from thread\n");
                }
            });
            threads.push(thr);
        }

        for thr in threads {
            let _ = thr.join();
        }
    }
}

fn main() {
    let handler = Handler::new("insecure_log.txt");
    handler.run();
}