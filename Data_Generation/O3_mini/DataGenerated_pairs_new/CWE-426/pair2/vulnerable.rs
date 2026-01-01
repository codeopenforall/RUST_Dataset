use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Loader;

impl Loader {
    pub fn new() -> Self {
        Loader
    }

    // Executes an external command using a relative path.
    // This unsafe block and relative path usage allow an attacker to
    // control which executable is loaded, reflecting CWE-426.
    pub fn run(&self, cmd: &str, args: &[&str]) -> Result<String, String> {
        let result = unsafe {
            Command::new(cmd)
                .args(args)
                .output()
        };

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
            // Vulnerable usage: using a relative command name without ensuring an absolute path.
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