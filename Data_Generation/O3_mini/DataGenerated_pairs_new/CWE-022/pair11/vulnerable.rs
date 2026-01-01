//////////////////////////////////////////////
// Vulnerability Example: CWE-022
//////////////////////////////////////////////
use std::env;
use std::fs;
use std::path::PathBuf;
use std::io;
use std::thread;

// A struct representing a file loader with a designated safe directory.
struct Loader {
    root: PathBuf,
}

impl Loader {
    // Constructs a new instance with the provided safe directory.
    fn new(root: PathBuf) -> Self {
        Loader { root }
    }

    // Reads the content of a file by appending the provided path to the safe root.
    // NOTE: This function uses a naive join without canonicalization and permits path traversal.
    fn load(&self, file: &str) -> io::Result<String> {
        // Vulnerability: Directly joining user input without validating the final path.
        let full_path = self.root.join(file);
        let contents = fs::read_to_string(&full_path)?;

        // Unsafe block simulating low-level processing of file content.
        unsafe {
            let length = contents.len();
            let ptr = contents.as_ptr();
            let mut sum: u8 = 0;
            // Iterate through the bytes in an unsafe manner.
            for i in 0..length {
                let byte = *ptr.add(i);
                sum = sum.wrapping_add(byte);
            }
            // Dummy conditional branch.
            if sum == 0 {
                // No-op.
            }
        }
        Ok(contents)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    // Assume the safe directory is "./safe_dir"
    let loader = Loader::new(PathBuf::from("./safe_dir"));
    // Attempt to load the file specified by the user.
    match loader.load(&args[1]) {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error loading file: {:?}", e),
    }
}