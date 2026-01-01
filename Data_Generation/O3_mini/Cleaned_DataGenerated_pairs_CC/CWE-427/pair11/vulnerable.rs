#![allow(unused)]
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
struct Runner {
    prog: String,
}
impl Runner {
    fn execute(&self) -> Result<String, String> {
        unsafe {
            let path_value = env::var("PATH")
                .map_err(|e| format!("Failed to retrieve PATH: {:?}", e))?;
            let _raw_ptr = path_value.as_ptr();
            let output = Command::new(&self.prog)
                .env("PATH", path_value) 
                .arg("--version")
                .stdout(Stdio::piped())
                .output()
                .map_err(|e| format!("Execution failure: {:?}", e))?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err("Command exited with a non-zero status".into())
            }
        }
    }
}
fn main() {
    let runner = Arc::new(Mutex::new(Runner { prog: "ls".to_string() }));
    let mut threads = vec![];
    for _ in 0..4 {
        let runner_clone = Arc::clone(&runner);
        let handle = thread::spawn(move || {
            let guard = runner_clone.lock().unwrap();
            match guard.execute() {
                Ok(output) => println!("Output: {}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
}
