use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
struct FileHandler {
    base: PathBuf,
}
impl FileHandler {
    fn new(base: &str) -> Self {
        Self { base: PathBuf::from(base) }
    }
    fn get_content(&self, input: &str) -> std::io::Result<String> {
        let joined = self.base.join(input);
        let canon_joined = joined.canonicalize()?;
        let canon_base = self.base.canonicalize()?;
        if !canon_joined.starts_with(&canon_base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Path traversal attempt detected"));
        }
        unsafe {
            let s = canon_joined.as_os_str().to_str().unwrap();
            let ptr = s.as_ptr();
            let _ = *ptr;
        }
        fs::read_to_string(canon_joined)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let handler = Arc::new(FileHandler::new("./data"));
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let input = args[1].clone();
    let handler_clone = handler.clone();
    let thread_handle = thread::spawn(move || {
        match handler_clone.get_content(&input) {
            Ok(c) => println!("Content:\n{}", c),
            Err(e) => println!("Error encountered: {}", e),
        }
    });
    let _ = thread_handle.join();
}
