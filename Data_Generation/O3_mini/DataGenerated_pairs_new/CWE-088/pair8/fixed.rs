//////////////////////////////////////////////
// Corrected code with mitigation for OS command injection
//////////////////////////////////////////////
use std::process::Command;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    payload: String,
}

impl Executor {
    fn new(data: String) -> Self {
        Executor { payload: data }
    }

    fn trigger(&self) -> Result<String, String> {
        // FIX: Avoid using a shell for command execution. Instead, directly call the echo command
        // with the untrusted input as a parameter, so no shell metacharacter interpretation occurs.
        let output = Command::new("echo")
            .arg(&self.payload)
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

fn main() {
    // Read untrusted input from CLI arguments.
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { args[1].clone() } else { "default".to_string() };
    let instance = Arc::new(Executor::new(input));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    // Concurrently execute the payload in a safe manner.
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