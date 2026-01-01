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
    pub unsafe fn open_file(&self) -> std::io::Result<File> {
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o777)
            .open(&self.path)
    }
    pub fn write_data(&self, data: &str) -> std::io::Result<()> {
        let guard = Arc::new(Mutex::new(()));
        let file_path = self.path.clone();
        let guard_clone = guard.clone();
        let data_string = data.to_owned();
        let handle = thread::spawn(move || {
            let _lock = guard_clone.lock().unwrap();
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
    pub fn unsafe_operation(&self) {
        unsafe {
            let mut value: i32 = 100;
            let ptr: *mut i32 = &mut value;
            *ptr += 10; 
            println!("Unsafe operation result: {}", value);
        }
    }
}
pub fn run(file_path: &str) {
    let handler = Handler::new(file_path);
    unsafe {
        handler.open_file().expect("Failed to open file unsafely");
    }
    handler.write_data("Vulnerable data write\n").expect("Write failed");
    handler.unsafe_operation();
}
fn main() {
    run("/tmp/insecure.txt");
}
