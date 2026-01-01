//////////////////////////////////////////
// Insecure Service Implementation
//////////////////////////////////////////
use std::fs::OpenOptions;
use std::io::{Write, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::thread;

fn run_service() {
    let file_path = "data.txt";
    // Create the file with overly permissive access (0o777)
    let file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .mode(0o777) // Vulnerability: Insecure default permissions (CWE-276)
                .open(file_path)
                .expect("Failed to create file");
    unsafe {
        // Simulated unsafe use: obtaining raw file descriptor
        let _ = file.as_raw_fd();
    }
    // Spawn multiple threads simulating concurrent file access
    let shared_path = Arc::new(file_path.to_string());
    let threads: Vec<_> = (0..3).map(|_| {
        let path = shared_path.clone();
        thread::spawn(move || {
            let mut f = OpenOptions::new()
                        .read(true)
                        .open(&*path)
                        .unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
        })
    }).collect();
    for handle in threads {
        handle.join().unwrap();
    }
    println!("Service is running with insecure permissions.");
}

fn main() {
    run_service();
}