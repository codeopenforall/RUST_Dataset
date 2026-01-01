use std::process::Command;
use std::thread;
use std::sync::Arc;
trait Executor {
    fn execute(&self) -> Result<(), String>;
}
struct Launcher {
    path: String,
}
impl Executor for Launcher {
    fn execute(&self) -> Result<(), String> {
        unsafe {
            let status = Command::new(&self.path)
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
    let launcher = Arc::new(Launcher { path: "vulnerable_app".to_owned() });
    let launcher_clone = Arc::clone(&launcher);
    let handle = thread::spawn(move || {
        let _ = launcher_clone.execute();
    });
    handle.join().unwrap();
}
fn main() {
    run();
}
