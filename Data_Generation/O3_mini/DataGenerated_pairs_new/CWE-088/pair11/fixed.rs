//////////////////////////////////////////////////////////////
// Corrected Code: Safe Command Execution Avoiding Shell Injection   //
// CWE-088 mitigation: Do not pass unsanitized input to the shell.
//////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::Arc;
use std::thread;

fn execute_command(input: &str) -> std::io::Result<String> {
    // Instead of invoking the shell, we directly call the echo command with the untrusted
    // input as an argument. This avoids shell interpretation of any metacharacters.
    Command::new("echo")
        .arg(input)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn main() {
    // Read untrusted input from the command line.
    let input = std::env::args().nth(1).expect("Expected input argument");
    let shared_input = Arc::new(input);
    let mut handles = vec![];

    // Spawn threads that concurrently call the safe command execution.
    for _ in 0..4 {
        let cloned_input = Arc::clone(&shared_input);
        handles.push(thread::spawn(move || {
            match execute_command(&cloned_input) {
                Ok(result) => result,
                Err(e) => format!("Error: {}", e),
            }
        }));
    }

    // Print each thread's output.
    for handle in handles {
        println!("{}", handle.join().unwrap());
    }
}