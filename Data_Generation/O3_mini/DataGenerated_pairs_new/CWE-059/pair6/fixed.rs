////////////////////////////////////////////////////////////////////////////////
// Corrected Version – This implementation defends against the symbolic link attack
// by canonicalizing the user-supplied path and ensuring that the resulting absolute
// path is within a trusted allowed directory. Any attempt to supply a symlink that escapes
// the allowed directory is rejected.
////////////////////////////////////////////////////////////////////////////////
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use std::io;

struct FileHandler {
    allowed: PathBuf,
}

impl FileHandler {
    fn write_data(&self, user_path: &Path, data: &str) -> io::Result<()> {
        // First, obtain the canonical (absolute) path of the user input.
        let canonical = user_path.canonicalize()?;
        // Then, ensure that the canonical path is within the allowed directory.
        if !canonical.starts_with(&self.allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access denied: Invalid path"));
        }
        fs::write(canonical, data)
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