////////////////////////////////////////////////////////////////////////////////
// Vulnerable Version – This implementation mistakenly trusts a user‐supplied
// file path and uses an unsafe block to “manipulate” it via raw pointer
// arithmetic without performing proper checks on symbolic link resolution.
// As a result, an attacker can supply a symbolic link that points outside the
// allowed directory, thereby accessing or modifying unintended files.
////////////////////////////////////////////////////////////////////////////////
use std::ffi::{CString, CStr};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;

struct FileHandler {
    allowed: PathBuf,
}

impl FileHandler {
    // The write_data method uses an unsafe block that converts the input
    // path to a C string and back. No validation is done to check if the
    // resolved path (after possible symlink indirection) lies within the
    // permitted directory.
    fn write_data(&self, user_path: &Path, data: &str) -> io::Result<()> {
        unsafe {
            // Convert the provided file path to a C string unsafely
            let c_path = CString::new(user_path.to_str().unwrap()).unwrap();
            let ptr = c_path.as_ptr();
            // Simulate unsafe pointer arithmetic (demonstrative only)
            let manipulated_ptr = ptr.offset(0);
            let manipulated = CStr::from_ptr(manipulated_ptr).to_string_lossy().into_owned();
            // Vulnerability: The manipulated path is used directly without checking
            fs::write(manipulated, data)
        }
    }
}

fn main() {
    let allowed = env::temp_dir().join("allowed_dir");
    let _ = fs::create_dir_all(&allowed);
    let handler = FileHandler { allowed };
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <program> <file_path>");
        return;
    }
    let user_file = Path::new(&args[1]);
    match handler.write_data(user_file, "Confidential Data") {
        Ok(_)  => println!("Data written successfully."),
        Err(e) => eprintln!("Error writing data: {}", e),
    }
}