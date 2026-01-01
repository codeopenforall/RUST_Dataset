use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Executor {
    data: Arc<Mutex<String>>,
}
impl Executor {
    pub fn new(initial: String) -> Self {
        Executor {
            data: Arc::new(Mutex::new(initial)),
        }
    }
    pub fn run(&self) -> Result<String, String> {
        let safe_input = {
            let locked = self.data.lock().unwrap();
            locked.clone()
        };
        let output = Command::new("echo")
                        .arg(safe_input)
                        .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        args[1].clone()
    } else {
        "default".to_string()
    };
    let executor = Executor::new(input);
    let mut handles = vec![];
    for _ in 0..4 {
        let ex_clone = Executor {
            data: Arc::clone(&executor.data),
        };
        let handle = thread::spawn(move || {
            let res = ex_clone.run();
            println!("Thread result: {:?}", res);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
