/////////////////////// Fixed Source Code ///////////////////////
// This revised code eliminates the untrusted search path vulnerability by constructing an absolute path
// to the trusted executable. It does so by assuming that the valid helper is located in a secure subdirectory "trusted"
// relative to the application's installation directory.
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
use std::path::{Path, PathBuf};
use std::fs;

struct Executor {
    command: String,
}

impl Executor {
    // The execute method now builds an absolute path to the trusted executable.
    // It prevents loading an executable from untrusted locations by explicitly specifying the secure directory.
    fn execute(&self) -> Result<(), String> {
        // Determine the directory of the current executable.
        let exe_path = env::current_exe().map_err(|e| format!("Current exe error: {}", e))?;
        let parent_dir = exe_path.parent().ok_or_else(|| "Failed to determine exe directory".to_string())?;
        // Construct the absolute path: assume the trusted helper is in a "trusted" subdirectory.
        let safe_path: PathBuf = [parent_dir, Path::new("trusted"), Path::new(&self.command)].iter().collect();
        let safe_str = safe_path.to_str().ok_or_else(|| "Invalid path string".to_string())?;
        unsafe {
            let mut child = Command::new(safe_str)
                .spawn()
                .map_err(|e| format!("Spawn error: {}", e))?;
            let status = child.wait().map_err(|e| format!("Wait error: {}", e))?;
            if status.success() {
                Ok(())
            } else {
                Err("Process did not exit successfully".to_string())
            }
        }
    }
}

fn main() {
    // The Executor is configured with the same command name,
    // but due to the absolute path resolution it will search for the helper in the secure "trusted" subdirectory.
    let exec = Arc::new(Mutex::new(Executor {
        command: "helper".to_string(),
    }));
    let mut handles = vec![];

    // Spawn threads to concurrently execute the trusted helper.
    for _ in 0..3 {
        let exec_clone = Arc::clone(&exec);
        let handle = thread::spawn(move || {
            let instance = exec_clone.lock().unwrap();
            instance.execute().unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}