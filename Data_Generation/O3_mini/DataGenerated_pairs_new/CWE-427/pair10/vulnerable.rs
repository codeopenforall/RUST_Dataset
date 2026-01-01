//// Vulnerable Code Example for CWE-427
// This code unsafely propagates an externally-controlled environment variable
// ("UNTRUSTED_PATH") into the PATH of a spawned process, without validating its contents.
// It also employs an unsafe block to convert the string’s internal pointer which may lead
// to undefined behavior if the string is not null-terminated.
// The design uses a struct and concurrency to mimic realistic patterns.
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

    // Executes a specified command and returns its standard output as a String.
    pub fn run(&self, cmd: &str) -> std::io::Result<String> {
        // Read an externally controlled environment variable.
        let untrusted = env::var("UNTRUSTED_PATH").unwrap_or_else(|_| "/usr/bin".to_string());

        // Unsafe block that unsafely interprets the string's internal pointer.
        // If the untrusted input isn't null-terminated, this leads to undefined behavior.
        unsafe {
            let ptr = untrusted.as_ptr() as *const i8;
            // The unsound use of CStr::from_ptr() assumes there is a valid null terminator.
            let _ = CStr::from_ptr(ptr);
        }

        // Spawning a process using the externally-provided PATH without sanitization.
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
        // Using "env" to output the process environment,
        // which will include the unsanitized PATH variable.
        let output = exec_clone.run("env").expect("Failed to run command");
        println!("{}", output);
    });
    handle.join().expect("Thread panicked");
}