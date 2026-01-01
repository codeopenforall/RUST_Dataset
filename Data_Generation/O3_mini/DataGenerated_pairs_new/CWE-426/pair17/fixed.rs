// Corrected sample mitigating CWE-426 by enforcing absolute path usage.
// The code now checks that the provided command path is absolute and canonicalizes it,
// ensuring that only executables from trusted locations are run. The unsafe and concurrency
// constructs are retained to mirror the production code structure.
use std::process::Command;
use std::thread;
use std::sync::Arc;
use std::path::Path;

trait Executor {
    fn execute(&self) -> Result<(), String>;
}

struct Launcher {
    path: String,
}

impl Executor for Launcher {
    fn execute(&self) -> Result<(), String> {
        // Enforce usage of absolute paths.
        let p = Path::new(&self.path);
        if !p.is_absolute() {
            return Err("Relative paths are rejected".to_owned());
        }
        // Canonicalize to resolve symbolic links, etc.
        let abs_path = p.canonicalize().map_err(|e| e.to_string())?;
        unsafe {
            let status = Command::new(abs_path)
                .arg("--version")
                .status()
                .map_err(|e| e.to_string())?;
            if status.success() {
                Ok(())
            } else {
                Err("Command execution failed".to_owned())
            }
        }
    }
}

fn run() {
    // Provide an absolute path to the trusted executable.
    let launcher = Arc::new(Launcher { path: "/usr/bin/vulnerable_app".to_owned() });
    let launcher_clone = Arc::clone(&launcher);
    let handle = thread::spawn(move || {
        let _ = launcher_clone.execute();
    });
    handle.join().unwrap();
}

fn main() {
    run();
}