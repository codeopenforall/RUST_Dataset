use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};
use std::path::{PathBuf};
use std::sync::Arc;
use std::thread;
struct FileServer {
    root: PathBuf,
}
impl FileServer {
    fn new(root: PathBuf) -> Self {
        FileServer { root }
    }
    fn serve(&self, req: &str) -> Result<Vec<u8>> {
        let candidate = self.root.join(req);
        let real_candidate = candidate.canonicalize()?;
        let safe_root = self.root.canonicalize()?;
        if !real_candidate.starts_with(&safe_root) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }
        unsafe {
            let mut file = File::open(&real_candidate)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
}
fn main() {
    let server = Arc::new(FileServer::new(PathBuf::from("./safe_dir")));
    let requests = vec!["../Cargo.toml", "data.txt"];
    let handles: Vec<_> = requests.into_iter().map(|req| {
        let srv = Arc::clone(&server);
        thread::spawn(move || {
            match srv.serve(req) {
                Ok(content) => println!("Served {} bytes for request {:?}", content.len(), req),
                Err(e) => println!("Error serving request {:?}: {:?}", req, e),
            }
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
}
