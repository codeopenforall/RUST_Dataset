#![allow(unused)]
use std::env;
use std::thread;

struct Runner;

impl Runner {
    fn execute(&self) -> Result<String, String> {
        // Unsafe read of environment variable "SEARCH_PATH"
        // CWE-427: Uncontrolled Search Path Element - unsafely using user-controlled input.
        let user_input = unsafe {
            let ptr = env::var("SEARCH_PATH").ok().map(|s| {
                let boxed = Box::new(s);
                Box::into_raw(boxed)
            });
            let result = match ptr {
                Some(raw) => {
                    // Use raw pointer without proper validation.
                    let string_ref = &*raw;
                    string_ref.clone()
                }
                None => String::new(),
            };
            result
        };

        // Concurrency: Spawn a thread that sets the global PATH based on unsanitized, attacker-controlled input.
        let handle = thread::spawn(move || {
            env::set_var("PATH", user_input);
        });
        handle.join().unwrap();

        // Return the current PATH value.
        let current_path = env::var("PATH").unwrap_or_default();
        Ok(current_path)
    }
}

fn main() {
    let runner = Runner;
    match runner.execute() {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }
}