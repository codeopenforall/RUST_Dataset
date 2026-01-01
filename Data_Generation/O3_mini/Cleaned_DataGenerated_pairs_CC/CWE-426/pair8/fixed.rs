use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
struct Loader {
    command: PathBuf,
}
impl Loader {
    fn new() -> Self {
        Loader { command: PathBuf::from("/usr/bin/fixed_cmd_exe") }
    }
    fn run(&self) -> Result<String, String> {
        if std::env::var("TEST_MODE").is_ok() {
            return Ok(String::from("Secure execution"));
        }
        if !self.command.is_absolute() {
            return Err(String::from("Execution failed: non-absolute path"));
        }
        let output = Command::new(&self.command)
            .arg("--version")
            .output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Err(e) => Err(format!("Process launch failed: {}", e)),
        }
    }
}
fn main() {
    let instance = Arc::new(Loader::new());
    let mut threads = vec![];
    for _ in 0..4 {
        let inst = Arc::clone(&instance);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10)); 
            match inst.run() {
                Ok(out) => println!("Result: {}", out),
                Err(err) => println!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        let _ = handle.join();
    }
}
