use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Write, Read};
struct Manager {
    path: String,
    data: Arc<Mutex<String>>,
}
impl Manager {
    fn new(path: &str) -> Manager {
        Manager {
            path: path.to_string(),
            data: Arc::new(Mutex::new(String::new())),
        }
    }
    fn create_file(&self) -> std::io::Result<File> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .mode(0o600)
            .open(&self.path)?;
        Ok(file)
    }
    fn write_data(&self, content: &str) -> std::io::Result<()> {
        let mut file = self.create_file()?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    fn read_data(&self) -> std::io::Result<String> {
        let mut file = File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
fn main() {
    let manager = Manager::new("/tmp/test_insecure_file.txt");
    let manager_ref = Arc::new(manager);
    let mut handles = vec![];
    for _ in 0..3 {
        let manager_clone = Arc::clone(&manager_ref);
        let handle = thread::spawn(move || {
            let data = "Sensitive Data";
            let _ = manager_clone.write_data(data);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let content = manager_ref.read_data().unwrap();
    println!("File content: {}", content);
}
