//////////////////////////////////////////////////////////////////////////
// Corrected implementation that avoids OS command injection by not using the shell
//////////////////////////////////////////////////////////////////////////
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor {
    input: String,
    state: Arc<Mutex<i32>>,
}

impl Executor {
    fn new(input: String) -> Self {
        Executor { input, state: Arc::new(Mutex::new(0)) }
    }
    
    fn run(&self) -> String {
        // Fixed: Directly pass the input as an argument to "echo".
        // This prevents shell interpretation and thus blocks command injection.
        let output = Command::new("echo")
                .arg(&self.input)
                .output()
                .expect("failed to execute process");
        // Properly update shared state using safe locking.
        {
            let mut guard = self.state.lock().unwrap();
            *guard += 1;
        }
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}

fn simulate() -> String {
    // The same input is used; however, metacharacters are treated as literal characters.
    let executor = Executor::new(String::from("injected_text; echo injection"));
    let state_clone = executor.state.clone();
    let handle = thread::spawn(move || {
        // Accessing shared state safely.
        let mut num = state_clone.lock().unwrap();
        *num += 1;
    });
    let result = executor.run();
    handle.join().unwrap();
    result
}

fn main() {
    let result = simulate();
    println!("Result: {}", result);
}