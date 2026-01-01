//////////////////////////////////////////////
// Vulnerable code susceptible to CWE-088
//////////////////////////////////////////////
use std::process::Command;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CStr};

struct Executor {
    payload: String,
}

impl Executor {
    fn new(data: String) -> Self {
        Executor { payload: data }
    }

    fn trigger(&self) -> Result<String, String> {
        // UNSAFE usage: convert Rust string to C string pointer without ensuring NUL termination.
        // Then, unsafely use it to build a shell command vulnerable to injection.
        unsafe {
            // intentionally take pointer to inner data (which is not null terminated) for simulation purposes
            let raw_ptr = self.payload.as_ptr() as *const i8;
            // The following call assumes the pointer is at least null-terminated.
            // An attacker might craft a payload containing shell metacharacters to inject commands.
            let cstr_payload = CStr::from_ptr(raw_ptr);
            // Build a command string by directly embedding the untrusted payload.
            let command_line = format!("echo {}", cstr_payload.to_str().unwrap_or(""));
            // Invoke the shell with -c option.
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_line)
                .output();
            match output {
                Ok(o) => {
                    let text = String::from_utf8_lossy(&o.stdout).into_owned();
                    Ok(text)
                },
                Err(e) => Err(format!("Execution error: {}", e)),
            }
        }
    }
}

fn main() {
    // Read untrusted input from CLI arguments.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { args[1].clone() } else { "default".to_string() };
    let instance = Arc::new(Executor::new(input));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    // Concurrently execute the payload.
    for _ in 0..5 {
        let exe_clone = Arc::clone(&instance);
        let res_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let result = exe_clone.trigger();
            res_clone.lock().unwrap().push(result);
        });
        threads.push(handle);
    }

    for t in threads {
        t.join().unwrap();
    }
}