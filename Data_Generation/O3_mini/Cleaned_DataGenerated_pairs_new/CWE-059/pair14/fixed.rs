use std::path::{Path, PathBuf};
use std::fs::File;
use std::io;
use std::sync::Arc;
use std::thread;
struct FileHandler {
    root: PathBuf,
}
impl FileHandler {
    fn new(root: PathBuf) -> Self {
        Self { root }
    }
    fn access(&self, relative: &str) -> io::Result<File> {
        let path = self.root.join(relative).canonicalize()?;
        if !path.starts_with(&self.root) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access denied: resolved path is outside the allowed directory",
            ));
        }
        File::open(path)
    }
}
fn main() {
    let base = PathBuf::from("/tmp/secure_dir");
    let handler = Arc::new(FileHandler::new(base));
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let h = Arc::clone(&handler);
            thread::spawn(move || {
                let _ = h.access("sample.txt");
            })
        })
        .collect();
    for t in threads {
        let _ = t.join();
    }
}
