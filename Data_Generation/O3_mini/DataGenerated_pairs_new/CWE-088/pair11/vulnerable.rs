//////////////////////////////////////////////////////////////
// Vulnerable Code: OS Shell Command Injection via unsafe use //
// CWE-088: Improper Neutralization of Special Elements used in an OS Command ('OS Command Injection')
//////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::Arc;
use std::thread;

fn execute_command(input: &str) -> std::io::Result<String> {
    // This function unsafely injects untrusted input into a shell command.
    // It concatenates the input with a fixed string and passes it to the shell.
    let cmd_line = format!("echo {}", input);
    unsafe {
        // Unsafe block simulating low-level interaction; the input is not sanitized.
        let c_str = std::ffi::CString::new(cmd_line).unwrap();
        Command::new("sh")
            .arg("-c")
            .arg(c_str.to_str().unwrap())
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

fn main() {
    // Read untrusted input from the command line.
    let input = std::env::args().nth(1).expect("Expected input argument");
    let shared_input = Arc::new(input);
    let mut handles = vec![];

    // Spawn threads that concurrently call the vulnerable command execution.
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