//////////////////////////////////////////////
// Vulnerable code for uncontrolled search path //
//////////////////////////////////////////////
use std::env;
use std::thread;
use libc;
use std::ffi::CString;

// A trait representing a command executor.
trait Executor {
    fn run(&self) -> String;
}

// A simple structure that implements the executor.
struct Worker;

impl Executor for Worker {
    fn run(&self) -> String {
        // Simulate an external command execution.
        // The returned value is determined by the PATH environment variable.
        let path = env::var("PATH").unwrap_or_default();
        if path.contains("attacker") {
            // Vulnerable behavior: if attacker controlled substring exists, return compromised result.
            "ATTACK".to_string()
        } else {
            "safe_output".to_string()
        }
    }
}

// This function unsafely updates the PATH environment variable based on an external input.
// It directly uses an attacker-controlled environment variable to change the search path.
fn update_env() {
    if let Ok(custom) = env::var("CUSTOM_PATH") {
        // UNSAFE: attacker-controlled value is passed directly to setenv without validation.
        unsafe {
            // Potential vulnerability: CWE-427 – Uncontrolled Search Path Element
            let c_custom = CString::new(custom).expect("CString::new failed");
            libc::setenv(b"PATH\0".as_ptr() as *const i8, c_custom.as_ptr(), 1);
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
    // Update PATH from an attacker-controlled variable without sanitization.
    update_env();
    let results = execute_threads();
    for res in results {
        println!("{}", res);
    }
}