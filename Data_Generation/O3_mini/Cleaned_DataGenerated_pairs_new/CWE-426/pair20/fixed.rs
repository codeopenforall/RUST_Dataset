use std::sync::{Arc, Mutex};
use std::thread;
trait Processor {
    fn process(&self) -> Result<String, String>;
}
struct Loader {
    path: String,
}
impl Loader {
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
pub fn run_process() -> String {
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
