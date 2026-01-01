////////////////////////////////////////////////////////////////////////////////
// Corrected Code Sample
////////////////////////////////////////////////////////////////////////////////
use std::path::{Component, Path, PathBuf};
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

    // Securely processes the input path by joining with the base directory,
    // then checking that the result does not contain any path traversal components.
    // This prevents the attacker from escaping the intended safe directory.
    fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        // Create a candidate path by joining the base and user input.
        let candidate = self.base.join(input);
        // Check each component of the candidate path.
        // If any component is a parent directory, reject the input.
        for comp in candidate.components() {
            if let Component::ParentDir = comp {
                return Err("Path traversal attempt detected".into());
            }
        }
        Ok(candidate)
    }
}

fn main() {
    // Define a directory intended to be safe.
    let safe = std::path::Path::new("safe_dir");
    let instance = Manager::new(safe);
    // Demonstrative input that attempts to escape the safe directory.
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