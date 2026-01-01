use std::env;
use std::process::Command;
use std::ffi::CStr;
use std::sync::Arc;
use std::thread;
pub struct Executor;
impl Executor {
    pub fn new() -> Self {
        Executor
    }
    pub fn run(&self, cmd: &str) -> std::io::Result<String> {
        let untrusted = env::var("UNTRUSTED_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        unsafe {
            let ptr = untrusted.as_ptr() as *const i8;
            let _ = CStr::from_ptr(ptr);
        }
        let mut command = Command::new(cmd);
        command.env("PATH", untrusted);
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
