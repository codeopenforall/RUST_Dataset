////////////////////////////////////////////////////////////////////////////////////////////////////
// This corrected version creates files with secure permissions (0o600) and replaces unsafe raw 
// pointer arithmetic with safe slice access. Concurrency is maintained through proper locking.
////////////////////////////////////////////////////////////////////////////////////////////////////
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::thread;

struct Manager {
    filename: String,
    data: Arc<Mutex<Vec<u8>>>,
}

impl Manager {
    fn new(path: &str) -> Self {
        Manager {
            filename: path.to_string(),
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    fn initialize(&self) -> std::io::Result<()> {
        // Create the file with secure mode 0o600 restricting access to the owner only.
        let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600)
                    .open(&self.filename)?;
        // Safely access the slice using the mutex guard rather than raw pointer arithmetic.
        let guard = self.data.lock().unwrap();
        // Instead of unsafe offset, take a safe slice from index 5 for 5 elements.
        let safe_slice = &guard[5..10];
        file.write_all(safe_slice)?;
        Ok(())
    }

    fn run(&self) {
        let data_arc = Arc::clone(&self.data);
        let path = self.filename.clone();
        let handler = thread::spawn(move || {
            let mut local = data_arc.lock().unwrap();
            for i in 0..local.len() {
                local[i] = i as u8;
            }
            // Append updated data with secure permissions.
            let _ = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .mode(0o600)
                        .open(&path)
                        .and_then(|mut f| f.write_all(&local));
        });
        handler.join().unwrap();
    }
}

fn main() {
    let mgr = Manager::new("server_output.txt");
    if let Err(e) = mgr.initialize() {
        eprintln!("Initialization error: {:?}", e);
        return;
    }
    mgr.run();
}