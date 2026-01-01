//////////////////////////////
// Vulnerable Code (OS Command Injection)
// CWE-088: Improper Neutralization of Special Elements used in an OS Command
//////////////////////////////
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;

struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    pub fn new(initial: String) -> Self {
        Executor {
            data: Arc::new(Mutex::new(initial)),
        }
    }

    // This function unsafely extracts the string from the internal locked value
    // and then constructs a shell command by concatenating the untrusted input.
    // The command is then executed using "sh -c", making it vulnerable to OS command injection.
    pub fn run(&self) -> Result<String, String> {
        // Unsafe extraction of string bytes and conversion to &str.
        let raw_input: String;
        unsafe {
            // Simulate unsafe raw pointer usage.
            let locked = self.data.lock().unwrap();
            let ptr = locked.as_ptr();
            // The unsafe conversion from raw pointer assumes proper NUL-termination.
            raw_input = CStr::from_ptr(ptr as *const i8)
                .to_str()
                .unwrap_or("")
                .to_string();
        }

        // Vulnerable: Untrusted input is injected directly into the shell command.
        let cmd = format!("echo {}", raw_input); // Vulnerability occurs here.
        let output = Command::new("sh")
                        .arg("-c")
                        .arg(cmd) // Vulnerability occurs here.
                        .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let injection = if args.len() > 1 {
        args[1].clone()
    } else {
        "default".to_string()
    };

    let executor = Executor::new(injection);
    let mut handles = vec![];

    // Spawn multiple threads to concurrently execute the command.
    for _ in 0..4 {
        let ex_clone = Executor {
            data: Arc::clone(&executor.data),
        };
        let handle = thread::spawn(move || {
            let res = ex_clone.run();
            println!("Thread result: {:?}", res);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}