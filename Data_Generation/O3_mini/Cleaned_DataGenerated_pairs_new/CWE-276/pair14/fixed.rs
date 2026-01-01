#![allow(dead_code)]
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::Write;
struct FileHandler {
    filepath: String,
}
impl FileHandler {
    unsafe fn create(&self) -> std::io::Result<File> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .mode(0o600)     
            .open(&self.filepath)?;
        Ok(file)
    }
    unsafe fn process(&self) -> std::io::Result<()> {
        let mut file = self.create()?;
        let buf: *mut u8 = &mut 42u8 as *mut u8;
        unsafe {
            *buf = 100;
        }
        file.write_all(b"data")?;
        Ok(())
    }
}
fn main() {
    let handler = Arc::new(Mutex::new(FileHandler { filepath: "output.txt".to_string() }));
    let mut handles = vec![];
    for _ in 0..4 {
        let h = Arc::clone(&handler);
        let th = thread::spawn(move || {
            let handler_lock = h.lock().unwrap();
            unsafe {
                handler_lock.process().unwrap();
            }
        });
        handles.push(th);
    }
    for th in handles {
        th.join().unwrap();
    }
    println!("Operation completed in secure mode.");
}
