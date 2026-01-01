////////////////////////////////////////////////////////////
// Corrected implementation for OS command injection CWE-088
////////////////////////////////////////////////////////////
use std::process::Command;
use std::thread;

struct Executor;

impl Executor {
    pub fn run(input: &str) -> Result<String, String> {
        // Validate the input: allow only alphanumeric characters and whitespace.
        // This prevents injection of shell metacharacters.
        if !input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
            return Err("Invalid characters in input".to_string());
        }
        // Safely build the command using strictly controlled input.
        let cmd_string = format!("echo {}", input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_string)
            .output()
            .map_err(|e| e.to_string())?;
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result.trim().to_string())
    }
}

fn main() {
    // Read input from command line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <input>");
        return;
    }
    // Spawn a thread to simulate concurrent execution.
    let user_input = args[1].clone();
    let handle = thread::spawn(move || {
        match Executor::run(&user_input) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}