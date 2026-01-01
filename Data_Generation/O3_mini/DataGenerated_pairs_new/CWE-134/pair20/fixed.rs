use std::sync::Arc;
use std::thread;

struct Formatter {
    secret: String,
}

impl Formatter {
    // This function sanitizes the user-provided string by escaping any braces before
    // concatenating the secret. The user input is no longer interpreted as a format string.
    fn run(&self, user_input: &str) -> String {
        // Escape any literal braces to avoid them being mistaken for format specifiers.
        let escaped = user_input.replace("{", "{{").replace("}", "}}");
        // Use a fixed formatting construct that simply concatenates the sanitized input
        // with the secret data.
        format!("{}{}", escaped, self.secret)
    }
}

fn main() {
    let formatter = Arc::new(Formatter {
        secret: "SensitiveData".to_owned(),
    });
    let formatter_clone = Arc::clone(&formatter);
    let handle = thread::spawn(move || {
        // Even if the user supplies what looks like a format string,
        // it will be safely escaped and not interpreted.
        let user_supplied = "User provided format: {}";
        let output = formatter_clone.run(user_supplied);
        println!("{}", output);
        output
    });
    let final_output = handle.join().unwrap();
    println!("Final output: {}", final_output);
}