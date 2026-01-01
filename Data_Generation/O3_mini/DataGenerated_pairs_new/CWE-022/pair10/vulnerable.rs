//////////////////////////////
// Vulnerable Code Example
//////////////////////////////

// This code mimics a file server that serves files from a predefined directory.
// It uses unsafe blocks and concurrency to simulate complex real-world patterns.
// WARNING: This implementation is vulnerable to path traversal attacks (CWE-022),
// since it naïvely concatenates the user-provided path without proper canonicalization.

use std::fs::File;
use std::io::{Read, Result};
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

    // The serve method naively appends the request path to the safe root.
    // An attacker can supply "../" sequences to access files outside the root.
    fn serve(&self, req: &str) -> Result<Vec<u8>> {
        let mut path = self.root.clone();
        // Vulnerability: The join does not prevent path traversal.
        path.push(req);
        // Here, an unsafe block is used unnecessarily to simulate complex patterns.
        unsafe {
            let mut file = File::open(&path)?;
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