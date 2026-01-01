use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Arc;
use std::thread;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
trait FileOperator {
    fn generate(&self) -> io::Result<()>;
}
struct FileManager {
    path: String,
}
impl FileManager {
    fn new(path: &str) -> Self {
        FileManager {
            path: path.to_owned(),
        }
    }
    unsafe fn insecure_setup(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)?;
        file.sync_all()?; 
        #[cfg(unix)]
        {
            let perms = PermissionsExt::from_mode(0o777);
            std::fs::set_permissions(&self.path, perms)?;
        }
        Ok(())
    }
}
impl FileOperator for FileManager {
    fn generate(&self) -> io::Result<()> {
        unsafe { self.insecure_setup() }
    }
}
fn main() {
    let manager = Arc::new(FileManager::new("data.txt"));
    let mut handles = Vec::new();
    for _ in 0..5 {
        let handler = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            let _ = handler.generate();
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    println!("Operation completed in insecure mode.");
}
