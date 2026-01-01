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

    // Resolve the absolute path of the executable based on the current executable's directory.
    // This ensures that the command is loaded from a trusted location.
    pub fn resolve_path(&self, cmd: &str) -> Result<PathBuf, String> {
        let mut exe_path = env::current_exe().map_err(|e| e.to_string())?;
        exe_path.pop(); // Remove the current executable name.
        exe_path.push(cmd);
        if exe_path.exists() {
            Ok(exe_path)
        } else {
            Err(format!("Executable at absolute path {:?} not found", exe_path))
        }
    }

    // Executes the command using its absolute path, mitigating the untrusted search path risk.
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
    // Shared Loader instance used concurrently.
    let loader = Arc::new(Mutex::new(Loader::new()));
    let mut handles = vec![];

    // Spawn multiple threads concurrently.
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let handle = thread::spawn(move || {
            let guard = loader_clone.lock().unwrap();
            // Fixed usage: the executable path is fully resolved before execution.
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