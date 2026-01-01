use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor {
    command: String,
}
impl Executor {
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
