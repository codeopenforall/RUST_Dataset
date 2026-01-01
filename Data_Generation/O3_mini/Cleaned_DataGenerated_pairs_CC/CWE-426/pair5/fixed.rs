#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
use std::process::Command;
use std::path::Path;
struct Processor {
    tool: String, 
}
impl Processor {
    fn new(tool: &str) -> Self {
        if !Path::new(tool).is_absolute() {
            panic!("Only absolute paths are allowed");
        }
        Processor { tool: tool.to_string() }
    }
    fn execute(&self) -> Result<String, String> {
        let output = Command::new(&self.tool).arg("--version").output();
        match output {
            Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
            Err(e) => Err(format!("Command failed: {}", e)),
        }
    }
}
fn main() {
    let runner = Arc::new(Processor::new("/usr/bin/safe_executable"));
    let handles: Vec<_> = (0..5).map(|_| {
        let r = Arc::clone(&runner);
        thread::spawn(move || {
            let res = r.execute();
            if res.is_err() {
                println!("Encountered an error running the command");
            }
        })
    }).collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("Program execution complete (corrected).");
}
