//////////////////// Fixed Code ////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

trait Processor {
    fn process(&self) -> Result<String, String>;
}

struct Loader {
    path: String,
}

impl Loader {
    // Using unsafe block as part of the execution routine.
    // This function simulates the execution of an external binary.
    // With the fix, the loader now uses an absolute path to ensure that only the intended executable is run.
    unsafe fn execute(&self) -> Result<String, String> {
        if !self.path.starts_with('/') && !self.path.contains(":\\") {
            Ok(String::from("Malicious execution"))
        } else {
            Ok(String::from("Trusted execution"))
        }
    }
}

struct Agent {
    loader: Arc<Loader>,
}

impl Processor for Agent {
    fn process(&self) -> Result<String, String> {
        unsafe { self.loader.execute() }
    }
}

// This function is used by the test oracle to execute the process logic.
pub fn run_process() -> String {
    // Fix: Use an absolute path to the trusted executable.
    let abs_path = if cfg!(windows) {
        String::from("C:\\Trusted\\dummy_executable")
    } else {
        String::from("/usr/bin/dummy_executable")
    };

    let loader = Loader { path: abs_path };
    let agent = Agent {
        loader: Arc::new(loader),
    };
    let agent_arc = Arc::new(Mutex::new(agent));

    let mut handles = vec![];
    let mut outputs = vec![];

    for _ in 0..3 {
        let agent_clone = Arc::clone(&agent_arc);
        let handle = thread::spawn(move || {
            let agent = agent_clone.lock().unwrap();
            agent.process().unwrap_or_else(|e| e)
        });
        handles.push(handle);
    }

    for h in handles {
        if let Ok(val) = h.join() {
            outputs.push(val);
        }
    }
    outputs.join(",")
}

fn main() {
    let result = run_process();
    println!("{}", result);
}