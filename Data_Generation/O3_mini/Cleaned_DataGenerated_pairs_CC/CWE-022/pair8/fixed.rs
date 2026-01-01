use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Error, ErrorKind};
struct Handler {
    base: PathBuf,
}
impl Handler {
    fn new(base: &Path) -> Self {
        Handler { base: base.to_path_buf() }
    }
    fn read_file(&self, filename: &str) -> Result<String, std::io::Error> {
        let joined = self.base.join(filename);
        let canon_base = fs::canonicalize(&self.base)?;
        let canon_target = fs::canonicalize(&joined)?;
        if !canon_target.starts_with(&canon_base) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }
        unsafe {
            let bytes = canon_target.to_str().unwrap().as_bytes();
            let ptr = bytes.as_ptr();
            let _ = *ptr;
        }
        fs::read_to_string(canon_target)
    }
}
use std::thread;
fn main() {
    let base = env::current_dir().unwrap().join("safe_dir");
    let _ = fs::create_dir_all(&base);
    let handler = Handler::new(&base);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = args[1].clone();
    let handle = thread::spawn({
        let handler = Handler { base: handler.base.clone() };
        move || {
            match handler.read_file(&filename) {
                Ok(content) => println!("File content:\n{}", content),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    });
    handle.join().unwrap();
}
