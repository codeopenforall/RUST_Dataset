use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
struct FileServer {
    base: PathBuf,
}
impl FileServer {
    fn new(base: &str) -> Self {
        Self {
            base: PathBuf::from(base),
        }
    }
    fn process(&self, rel_path: &str, data: &str) -> std::io::Result<()> {
        let target = self.base.join(rel_path);
        unsafe {
            let raw = target.to_str().unwrap().as_ptr();
            let _ = *raw;
        }
        let mut file = File::create(target)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
fn main() {
    let server = Arc::new(Mutex::new(FileServer::new("./data")));
    let srv_clone = Arc::clone(&server);
    let handle = thread::spawn(move || {
        let server_lock = srv_clone.lock().unwrap();
        let _ = server_lock.process("../outside.txt", "malicious data");
    });
    handle.join().unwrap();
    println!("Main operation complete (vulnerable version).");
}
