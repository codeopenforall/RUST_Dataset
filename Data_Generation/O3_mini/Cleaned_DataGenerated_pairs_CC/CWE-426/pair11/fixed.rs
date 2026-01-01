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
    fn execute(&self) -> Result<(), String> {
        let exe_path = env::current_exe().map_err(|e| format!("Current exe error: {}", e))?;
        let parent_dir = exe_path.parent().ok_or_else(|| "Failed to determine exe directory".to_string())?;
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
    let exec = Arc::new(Mutex::new(Executor {
        command: "helper".to_string(),
    }));
    let mut handles = vec![];
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
