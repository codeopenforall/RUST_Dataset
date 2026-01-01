////////////////////////////
// Secure Code Sample     //
////////////////////////////
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

    // Secure file creation using strict permission mode.
    fn initiate(&self) -> std::io::Result<()> {
        // Even though an unsafe block is maintained to mimic real usage context,
        // we now specify a secure file mode.
        unsafe {
            let mut opts = OpenOptions::new();
            // Secure file mode: 0o640 (owner read/write, group read only, others no access)
            opts.create(true)
                .write(true)
                .truncate(true)
                .mode(0o640);
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
    // Spawning a thread to process file operations concurrently.
    let join_handle = thread::spawn(move || handler.initiate());
    join_handle.join().unwrap()
}

fn main() {
    // File management via a secure file handler.
    let handler = FileHandler::new("testfile.txt");
    worker(handler).expect("Thread execution failed");

    let reader = FileHandler::new("testfile.txt");
    let content = reader.retrieve().expect("Could not read file");
    println!("File content: {}", String::from_utf8_lossy(&content));
}