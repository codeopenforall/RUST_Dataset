use std::sync::Arc;
use std::thread;

struct Formatter {
    secret: String,
}

impl Formatter {
    // This function takes a user‐controlled format string and unsafely interprets
    // any occurrence of "{}" as a placeholder for sensitive data.
    fn run(&self, user_input: &str) -> String {
        // UNSAFE: the user_input is assumed to be valid UTF-8 and is used directly.
        unsafe {
            // In a real-world scenario, casting or leaking the user_input to a 'static
            // lifetime might be performed to bypass intended restrictions.
            let fmt = user_input;
            let mut result = String::new();
            let mut remainder = fmt;
            // Vulnerability: using user-controlled format string to perform formatting
            // substitutions leaks sensitive data if the string contains "{}" specifiers.
            while let Some(pos) = remainder.find("{}") {
                result.push_str(&remainder[..pos]);
                result.push_str(&self.secret);
                remainder = &remainder[pos + 2..];
            }
            result.push_str(remainder);
            result
        }
    }
}

fn main() {
    let formatter = Arc::new(Formatter {
        secret: "SensitiveData".to_owned(),
    });
    let formatter_clone = Arc::clone(&formatter);
    let handle = thread::spawn(move || {
        // Simulated externally-controlled format string from user
        let user_supplied = "User provided format: {}";
        let output = formatter_clone.run(user_supplied);
        println!("{}", output);
        output
    });
    let final_output = handle.join().unwrap();
    println!("Final output: {}", final_output);
}