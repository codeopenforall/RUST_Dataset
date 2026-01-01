use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor {
    directory: String,
}
impl Executor {
    fn new(input: String) -> Self {
        if input.contains(';') || input.contains('&') || input.contains('|') {
            panic!("Invalid input detected");
        }
        Executor { directory: input }
    }
    fn run(&self) -> Result<String, String> {
        let output = Command::new("ls")
            .arg(&self.directory)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 { args[1].clone() } else { String::new() };
    let executor = Executor::new(input);
    let shared_executor = Arc::new(Mutex::new(executor));
    let mut handles = vec![];
    for _ in 0..3 {
        let exec_clone = Arc::clone(&shared_executor);
        let handle = thread::spawn(move || {
            let guard = exec_clone.lock().unwrap();
            match guard.run() {
                Ok(output) => output,
                Err(err) => err,
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        println!("{}", handle.join().unwrap());
    }
}
