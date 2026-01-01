use std::process::Command;
use std::thread;
use std::env;
use std::path::Path;
use std::sync::Arc;
pub trait Runner {
    fn execute(&self) -> Result<(), String>;
}
pub struct Dispatch;
impl Dispatch {
    pub fn get_executable(&self) -> String {
        "/usr/local/bin/helper_bin".to_string()
    }
}
impl Runner for Dispatch {
    fn execute(&self) -> Result<(), String> {
        let exe = self.get_executable();
        if !Path::new(&exe).is_absolute() {
            return Err("Executable path must be absolute".to_string());
        }
        let exe_arc = Arc::new(exe);
        let exec_clone = Arc::clone(&exe_arc);
        let handle = thread::spawn(move || {
            unsafe {
                Command::new(exec_clone.as_str())
                    .arg("--version")
                    .spawn()
                    .map_err(|e| format!("spawn error: {}", e))
                    .and_then(|mut child| {
                        child.wait()
                            .map_err(|e| format!("wait error: {}", e))
                            .map(|_| ())
                    })
            }
        });
        handle.join().unwrap()
    }
}
pub fn run_app() -> Result<(), String> {
    let runner = Dispatch;
    runner.execute()
}
pub fn get_target() -> String {
    let d = Dispatch;
    d.get_executable()
}
fn main() {
    match run_app() {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => {
            eprintln!("Execution error: {}", e);
            std::process::exit(1);
        }
    }
}
