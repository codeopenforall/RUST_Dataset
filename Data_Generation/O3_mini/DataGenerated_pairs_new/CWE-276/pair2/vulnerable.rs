///////////////////////////////
// Vulnerability Code Sample //
///////////////////////////////
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::thread;

struct FileHandler {
    filename: String,
}

impl FileHandler {
    fn new(path: &str) -> Self {
        Self {
            filename: path.to_owned(),
        }
    }

    // Unsafe block wrapping file creation with overly permissive mode.
    fn initiate(&self) -> std::io::Result<()> {
        // Intentionally using an unsafe block (simulating complex real world usage)
        unsafe {
            let mut opts = OpenOptions::new();
            // Overly permissive file mode: 0o777
            opts.create(true)
                .write(true)
                .truncate(true)
                .mode(0o777);
            let mut file = opts.open(&self.filename)?;
            file.write_all(b"Sensitive internal data")?;
        }
        Ok(())
    }

    fn retrieve(&self) -> std::io::Result<Vec<u8>> {
        let mut file = OpenOptions::new().read(true).open(&self.filename)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }
}

fn worker(handler: FileHandler) -> std::io::Result<()> {
    // Spawning a thread to mimic concurrent file operations.
    let join_handle = thread::spawn(move || handler.initiate());
    join_handle.join().unwrap()
}

fn main() {
    // Using the file handler for file management in a concurrent setting.
    let handler = FileHandler::new("testfile.txt");
    // In a multi-threaded context.
    worker(handler).expect("Thread failed");

    let reader = FileHandler::new("testfile.txt");
    let content = reader.retrieve().expect("Could not read file");
    println!("File content: {}", String::from_utf8_lossy(&content));
}