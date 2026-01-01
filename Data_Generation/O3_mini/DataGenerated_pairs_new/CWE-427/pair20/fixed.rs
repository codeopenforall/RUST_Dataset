//////////////////////////////////////////////
// Corrected code for controlled search path  //
//////////////////////////////////////////////
use std::env;
use std::thread;

// A trait representing a command executor.
trait Executor {
    fn run(&self) -> String;
}

// A simple structure that implements the executor.
struct Worker;

impl Executor for Worker {
    fn run(&self) -> String {
        // Simulate an external command execution.
        // The behavior is based on the PATH environment variable.
        let path = env::var("PATH").unwrap_or_default();
        if path.contains("attacker") {
            // With proper sanitization the unsafe custom input is ignored.
            "ATTACK".to_string()
        } else {
            "safe_output".to_string()
        }
    }
}

// This function safely updates the PATH environment variable.
// It only allows override if the provided value matches an approved safe directory.
fn update_env() {
    if let Ok(custom) = env::var("CUSTOM_PATH") {
        // Sanitize the attacker-controlled value.
        // Only allow specific trusted directories.
        if custom == "/usr/bin" || custom == "/bin" {
            env::set_var("PATH", custom);
        } else {
            // Otherwise, ignore the custom input.
            // In a more sophisticated implementation, log or report an incident.
        }
    }
}

// Run a set of threads that all execute a command concurrently.
fn execute_threads() -> Vec<String> {
    let worker = Worker;
    let mut handles = vec![];
    for _ in 0..4 {
        // Each thread uses the same runner to perform a simulated command execution.
        let handle = thread::spawn(move || {
            worker.run()
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    // Safely update PATH after validating the custom input.
    update_env();
    let results = execute_threads();
    for res in results {
        println!("{}", res);
    }
}