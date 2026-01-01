-------------------------------------------------
use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::Command;

struct Executor {
    search_path: String,
}

impl Executor {
    // Only allow a whitelist of safe directories.
    fn new(input: String) -> Self {
        let allowed = vec!["/usr/bin", "/bin"];
        let sanitized = if allowed.contains(&input.as_str()) {
            input
        } else {
            // Fallback to a safe default.
            "/usr/bin".to_string()
        };
        Executor { search_path: sanitized }
    }

    // Sets the environment variable securely.
    fn update_env(&self) {
        env::set_var("PATH", &self.search_path);
    }

    // Executes the command using the safe std::process::Command API.
    fn run(&self) -> i32 {
        self.update_env();
        let output = Command::new("echo")
            .arg("safe_exec")
            .output()
            .expect("Failed to run command");
        // Return the command's exit code.
        output.status.code().unwrap_or(-1)
    }
}

fn worker(exec: Arc<Executor>, results: Arc<Mutex<Vec<i32>>>) {
    let ret = exec.run();
    let mut lock = results.lock().unwrap();
    lock.push(ret);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Use the first argument as the search path if provided.
    let input_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "/usr/bin".to_string()
    };

    let exec = Arc::new(Executor::new(input_path));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];

    // Spawn several threads to mimic concurrent execution.
    for _ in 0..3 {
        let exec_clone = Arc::clone(&exec);
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            worker(exec_clone, results_clone);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Thread panicked");
    }

    // Safely access the first result without raw pointer dereferencing.
    if let Some(&first) = results.lock().unwrap().get(0) {
        println!("Execution result: {}", first);
    }
    process::exit(0);
}
-------------------------------------------------