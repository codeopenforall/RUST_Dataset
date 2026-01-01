////////////////////////////////////////////////////////////////////////////////
// Vulnerable Code Sample
////////////////////////////////////////////////////////////////////////////////
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::thread;

struct Manager {
    base: PathBuf,
}

impl Manager {
    // Constructs a new Manager with a fixed base directory.
    fn new(base: &Path) -> Self {
        Manager {
            base: base.to_path_buf(),
        }
    }

    // This function joins the user-supplied input to the base.
    // Vulnerability: It uses a naive join without validating that the resulting
    // path is inside the base directory (allowing path traversal).
    // It also uses an unsafe block to simulate risky conversions.
    fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        // Naively join the input with the safe base.
        let joined = self.base.join(input);
        // Unsafe manipulation: convert the joined path to a CString and back.
        unsafe {
            let as_str = joined
                .to_str()
                .ok_or("Error: Invalid encoding in path")?;
            let c_string = CString::new(as_str)
                .map_err(|_| "Error: CString conversion failed")?;
            // Dangerous raw pointer conversion.
            let raw = c_string.into_raw();
            let recovered = CString::from_raw(raw);
            let result_str = recovered.to_str().map_err(|_| "Error: UTF-8 conversion failed")?;
            Ok(PathBuf::from(result_str))
        }
    }
}

fn main() {
    // Define a directory intended to be safe.
    let safe = std::path::Path::new("safe_dir");
    let instance = Manager::new(safe);
    // Demonstrative input that escapes the safe directory.
    let input = "../secret.txt";

    // Spawn a thread to simulate concurrent usage.
    let handle = thread::spawn({
        let inst = instance;
        move || {
            match inst.get_path(input) {
                Ok(path) => println!("Computed path: {:?}", path),
                Err(e) => println!("Error: {}", e),
            }
        }
    });
    handle.join().unwrap();
}