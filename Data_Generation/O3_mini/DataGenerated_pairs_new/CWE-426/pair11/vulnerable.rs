/////////////////////// Vulnerable Source Code ///////////////////////
// This code spawns a subprocess using a relative command name ("helper").
// It uses an unsafe block and concurrent threads with Arc/Mutex to simulate complex real‐world usage.
// The relative command causes the system to search in untrusted locations (e.g. the current directory)
// which can lead to loading an attacker-controlled executable.
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    command: String,
}

impl Executor {
    // The execute method spawns a process using a relative path.
    // Vulnerability: No absolute path resolution, so an attacker-controlled "helper" in an untrusted directory
    // may be executed.
    fn execute(&self) -> Result<(), String> {
        unsafe {
            let mut child = Command::new(&self.command)
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
    // Create a shared Executor with a relative command name.
    let exec = Arc::new(Mutex::new(Executor {
        command: "helper".to_string(),
    }));
    let mut handles = vec![];

    // Spawn several threads to execute the command concurrently.
    for _ in 0..3 {
        let exec_clone = Arc::clone(&exec);
        let handle = thread::spawn(move || {
            let instance = exec_clone.lock().unwrap();
            // In a real application, errors might be logged. Here we unwrap for brevity.
            instance.execute().unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}