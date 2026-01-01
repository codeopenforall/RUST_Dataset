use std::path::{Path, PathBuf};
use std::fs::{self, File};
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
        let mut path = self.root.join(relative);
        unsafe {
            let resolved = (*(&path as *const PathBuf)).canonicalize()?;
            path = resolved;
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
                let _ = h.access("..//etc/passwd");
            })
        })
        .collect();
    for t in threads {
        let _ = t.join();
    }
}
