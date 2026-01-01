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
        let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600)
                    .open(&self.filename)?;
        let guard = self.data.lock().unwrap();
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
