use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Loader;
impl Loader {
    pub fn new() -> Self {
        Loader
    }
    pub fn resolve_path(&self, cmd: &str) -> Result<PathBuf, String> {
        let mut exe_path = env::current_exe().map_err(|e| e.to_string())?;
        exe_path.pop(); 
        exe_path.push(cmd);
        if exe_path.exists() {
            Ok(exe_path)
        } else {
            Err(format!("Executable at absolute path {:?} not found", exe_path))
        }
    }
    pub fn run(&self, cmd: &str, args: &[&str]) -> Result<String, String> {
        let abs_cmd = self.resolve_path(cmd)?;
        let result = Command::new(abs_cmd)
            .args(args)
            .output();
        match result {
            Ok(out) => {
                if out.status.success() {
                    Ok(String::from_utf8_lossy(&out.stdout).to_string())
                } else {
                    Err("Command execution failed".to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute command: {}", e)),
        }
    }
}
fn main() {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let mut handles = vec![];
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let handle = thread::spawn(move || {
            let guard = loader_clone.lock().unwrap();
            guard.run("my_exe", &["--version"]).unwrap_or_else(|err| err)
        });
        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Ok(res) => println!("Thread result: {}", res),
            Err(_) => println!("Thread panicked"),
        }
    }
}
