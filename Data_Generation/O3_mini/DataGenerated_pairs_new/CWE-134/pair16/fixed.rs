////////////////////////////////////////////////////////////
// Fixed Code Sample
////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct LogContainer {
    // The sensitive value is retained in the struct but will no longer be exposed.
    secret: &'static str,
}

impl LogContainer {
    fn new() -> Self {
        LogContainer { secret: "VerySecretInfo" }
    }

    // The corrected process method uses only a fixed, literal format string.
    // The user input is treated solely as data, not as a format string.
    // This change prevents any externally controlled format string vulnerabilities.
    fn process(&self, user_data: &str) -> String {
        // Do not use the secret value here to avoid leaking it.
        // The fixed message simply echoes back the user provided data.
        format!("User message: {}", user_data)
    }
}

// Simulate concurrent operation similar to the vulnerable sample.
fn simulate(user_data: &str) -> String {
    let container = Arc::new(LogContainer::new());
    let output = Arc::new(Mutex::new(String::new()));

    let container_clone = Arc::clone(&container);
    let output_clone = Arc::clone(&output);

    let handle = thread::spawn(move || {
        let result = container_clone.process(user_data);
        let mut out_lock = output_clone.lock().unwrap();
        *out_lock = result;
    });

    handle.join().unwrap();
    let res = output.lock().unwrap();
    res.clone()
}

fn main() {
    // The external input is now only used as a data parameter.
    // This fixed design prevents any injection of format specifiers.
    let safe_input = "User provided data";
    let result = simulate(safe_input);
    println!("Result: {}", result);
}