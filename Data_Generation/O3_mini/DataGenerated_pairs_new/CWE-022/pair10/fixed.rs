//////////////////////////////
// Fixed Code Example
//////////////////////////////

// This corrected code implements a file server that restricts file access to a predefined
// directory. It performs canonicalization and verifies that the resolved path is within the safe directory,
// effectively mitigating CWE-022.
// Note: The unsafe block is retained to mimic legacy patterns, though it is now safe.

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

    // The serve method now canonicalizes the requested path and ensures it resides
    // under the expected safe root.
    fn serve(&self, req: &str) -> Result<Vec<u8>> {
        // First, join the root with the user provided path.
        let candidate = self.root.join(req);

        // Canonicalize both the candidate and the safe root.
        let real_candidate = candidate.canonicalize()?;
        let safe_root = self.root.canonicalize()?;

        // Check if the resolved candidate path starts with the safe root.
        if !real_candidate.starts_with(&safe_root) {
            return Err(Error::new(ErrorKind::PermissionDenied, "Access denied"));
        }

        // Unsafe block retained for legacy patterns.
        unsafe {
            let mut file = File::open(&real_candidate)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn main() {
    // Create a file server with a designated safe directory.
    let server = Arc::new(FileServer::new(PathBuf::from("./safe_dir")));
    let requests = vec!["../Cargo.toml", "data.txt"];

    // Spawn threads to simulate concurrent file accesses.
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