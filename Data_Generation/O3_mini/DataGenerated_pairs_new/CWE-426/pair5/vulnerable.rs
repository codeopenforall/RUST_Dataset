///////////////////////////
// Vulnerable Code Start //
///////////////////////////
#![allow(dead_code)]
use std::sync::Arc;
use std::thread;
use std::process::Command;

// A structure representing a command runner for external tools.
struct Processor {
    tool: String, // The tool is set using a relative path, posing a risk.
}

impl Processor {
    // Constructs a new Processor with the provided tool name.
    fn new(tool: &str) -> Self {
        // No absolute path enforcement: this accepts relative paths.
        Processor { tool: tool.to_string() }
    }

    // Executes the external tool and returns its output or an error message.
    fn execute(&self) -> Result<String, String> {
        // An unsafe block is used here to mimic low-level FFI interaction.
        unsafe {
            // Using Command::new with a relative tool name introduces an untrusted search path vulnerability.
            let output = Command::new(&self.tool).arg("--version").output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                Err(e) => Err(format!("Command failed: {}", e)),
            }
        }
    }
}

fn main() {
    // Instantiating Processor with a relative tool name, subject to untrusted search path exploitation.
    let runner = Arc::new(Processor::new("untrusted_executable"));
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
        // Joining the threads to ensure all executions complete.
        h.join().unwrap();
    }
    println!("Program execution complete (vulnerable).");
}
/////////////////////////
// Vulnerable Code End //
/////////////////////////