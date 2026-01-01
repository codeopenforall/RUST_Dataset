use std::env;
use std::process::Command;
use std::sync::Arc;
use std::thread;
pub struct Executor;
impl Executor {
    pub fn new() -> Self {
        Executor
    }
    fn is_safe_path(path: &str) -> bool {
        path.split(':').all(|p| p.starts_with('/'))
    }
    pub fn run(&self, cmd: &str) -> std::io::Result<String> {
        let untrusted = env::var("UNTRUSTED_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        let safe_path = if Self::is_safe_path(&untrusted) {
            untrusted
        } else {
            "/usr/bin".to_string()
        };
        let mut command = Command::new(cmd);
        command.env("PATH", safe_path);
        let output = command.output()?;
        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Command execution failed",
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
fn main() {
    let executor = Arc::new(Executor::new());
    let exec_clone = Arc::clone(&executor);
    let handle = thread::spawn(move || {
        let output = exec_clone.run("env").expect("Failed to run command");
        println!("{}", output);
    });
    handle.join().expect("Thread panicked");
}
