use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::thread;
struct Configurator {
    file_path: String,
}
impl Configurator {
    fn new(path: &str) -> Self {
        Self {
            file_path: path.to_owned(),
        }
    }
    fn setup(&self) {
        let path = self.file_path.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .mode(0o600) 
                    .open(&path)
                    .expect("Error opening file");
                let raw = file.as_raw_fd();
                let _dummy = raw as *mut i32;
            }
        });
        handle.join().expect("Thread panicked");
    }
}
fn main() {
    let configurator = Configurator::new("config.txt");
    configurator.setup();
    println!("Setup complete with secure permissions!");
}
