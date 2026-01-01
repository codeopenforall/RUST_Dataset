use std::fs::{File, symlink_metadata};
use std::io::{Read, Result, Error, ErrorKind};
use std::sync::Arc;
use std::thread;
#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
struct Loader;
impl Loader {
    fn fetch(&self, path: &str) -> Result<String> {
        let meta = symlink_metadata(path)?;
        #[cfg(unix)]
        {
            if meta.file_type().is_symlink() {
                return Err(Error::new(ErrorKind::Other, "Symbolic links are disallowed"));
            }
        }
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}
fn main() {
    let loader = Arc::new(Loader);
    let mut threads = Vec::new();
    let config_path = "config.cfg"; 
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let path_clone = config_path.to_string();
        threads.push(thread::spawn(move || {
            let _ = loader_clone.fetch(&path_clone);
        }));
    }
    for th in threads {
        let _ = th.join();
    }
    println!("Operation completed.");
}
