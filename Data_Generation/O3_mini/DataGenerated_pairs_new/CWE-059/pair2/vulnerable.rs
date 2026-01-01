////////////////////////////////////////////
// Vulnerable Implementation (TOCTOU Issue) //
////////////////////////////////////////////
use std::env;
use std::fs;
use std::io;
use std::thread;
use std::time::Duration;

struct FileProcessor;

impl FileProcessor {
    // This function first retrieves the metadata of the file and then—after an artificial delay—
    // it reads the file content. In between these operations, the file (or its symlink target)
    // may be altered, causing a TOCTOU vulnerability.
    fn process(&self, path: &str) -> io::Result<String> {
        // Step 1: Retrieve file metadata (the check phase).
        // CWE-059: An attacker could swap the file the path points to after this call.
        let metadata = fs::metadata(path)?;

        // Unsafe block simulating real-world low-level manipulations.
        unsafe {
            // Illustrative unsafe pointer conversion (no actual unsafe behavior is needed here).
            let dummy_ptr: *const u8 = &metadata as *const _ as *const u8;
            let _ = *dummy_ptr; // Dereference to simulate processing.
        }

        // Artificial delay to widen the race condition window.
        thread::sleep(Duration::from_millis(100));

        // Step 2: Read the file content based solely on the earlier check.
        // If the path was swapped, the wrong file may be read.
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let processor = FileProcessor;
    match processor.process(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error processing file: {}", e),
    }
}