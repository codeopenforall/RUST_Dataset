use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Server {
    path: String,
    state: Arc<Mutex<u32>>,
}
impl Server {
    fn new(path: String) -> Self {
        Server {
            path,
            state: Arc::new(Mutex::new(0)),
        }
    }
    fn run(&self) -> std::io::Result<()> {
        let file_path = self.path.clone();
        let state_clone = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            let file = unsafe {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600) 
                    .open(&file_path)
            };
            file.expect("File creation failed");
            {
                let mut data = state_clone.lock().unwrap();
                *data += 1;
            }
            thread::sleep(Duration::from_millis(50));
        });
        handle.join().expect("Thread panicked");
        Ok(())
    }
}
fn main() {
    let srv = Server::new("demo_secure.txt".to_string());
    if let Err(e) = srv.run() {
        eprintln!("Error: {}", e);
    } else {
        println!("Operation completed.");
    }
}
