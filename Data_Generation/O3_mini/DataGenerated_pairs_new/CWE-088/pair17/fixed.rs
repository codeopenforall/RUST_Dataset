//////////////////////////////
// Corrected Code (OS Command Injection Mitigation)
// The fix involves avoiding unsanitized shell command injection by passing user data
// directly as an argument to the echo command. It also avoids unsafe pointer dereferencing.
//////////////////////////////
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    data: Arc<Mutex<String>>,
}

impl Executor {
    pub fn new(initial: String) -> Self {
        Executor {
            data: Arc::new(Mutex::new(initial)),
        }
    }

    // Safely retrieves the user-provided string and executes the echo command
    // without invoking a shell to avoid command injection vulnerabilities.
    pub fn run(&self) -> Result<String, String> {
        let safe_input = {
            // Directly retrieve the string without unsafe raw pointer manipulation.
            let locked = self.data.lock().unwrap();
            locked.clone()
        };

        // Fixed: Instead of constructing a shell command, directly execute the echo binary with the
        // untrusted input as an argument. This prevents the shell from interpreting shell metacharacters.
        let output = Command::new("echo")
                        .arg(safe_input)
                        .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "default".to_string()
    };

    let executor = Executor::new(input);
    let mut handles = vec![];

    // Spawn multiple threads to concurrently run the command safely.
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