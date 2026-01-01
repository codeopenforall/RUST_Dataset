use std::{
    fs::File,
    io::{Error, Write},
    path::{Path, PathBuf},
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
        let base_canon = self.base.canonicalize()?;
        let target_canon = match target.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                target
                    .parent()
                    .and_then(|p| p.canonicalize().ok())
                    .map(|parent| parent.join(target.file_name().unwrap()))
                    .ok_or_else(|| Error::new(std::io::ErrorKind::Other, "Invalid path"))?
            }
        };
        if !target_canon.starts_with(&base_canon) {
            return Err(Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Access Denied: Path traversal detected",
            ));
        }
        unsafe {
            let raw = target_canon.to_str().unwrap().as_ptr();
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
        let res = server_lock.process("../outside.txt", "safe data");
        if res.is_err() {
            println!("Access denied as expected.");
        } else {
            println!("Unexpected success.");
        }
    });
    handle.join().unwrap();
    println!("Main operation complete (corrected version).");
}
