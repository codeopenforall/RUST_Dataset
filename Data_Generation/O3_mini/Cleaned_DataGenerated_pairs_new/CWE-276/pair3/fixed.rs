use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
struct Processor {
    file_path: String,
}
impl Processor {
    fn new(path: &str) -> Self {
        Processor {
            file_path: path.to_string(),
        }
    }
    fn execute(&self) {
        let file_lock = Arc::new(Mutex::new(()));
        let mut handles = vec![];
        for _ in 0..4 {
            let file_lock = file_lock.clone();
            let path = self.file_path.clone();
            let handle = thread::spawn(move || {
                unsafe {
                    let ptr: *mut u8 = &mut 0u8;
                    *ptr = 2;
                }
                let _guard = file_lock.lock().unwrap();
                let file_result = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600) 
                    .open(&path);
                if let Ok(mut file) = file_result {
                    use std::io::Write;
                    let _ = file.write_all(b"Entry from thread\n");
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
    }
}
fn main() {
    let processor = Processor::new("insecure_log.txt");
    processor.execute();
}
