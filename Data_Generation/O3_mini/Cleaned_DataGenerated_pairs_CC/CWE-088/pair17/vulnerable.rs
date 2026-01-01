use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CStr;
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
        let raw_input: String;
        unsafe {
            let locked = self.data.lock().unwrap();
            let ptr = locked.as_ptr();
            raw_input = CStr::from_ptr(ptr as *const i8)
                .to_str()
                .unwrap_or("")
                .to_string();
        }
        let cmd = format!("echo {}", raw_input); 
        let output = Command::new("sh")
                        .arg("-c")
                        .arg(cmd) 
                        .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(format!("Execution error: {}", e)),
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let injection = if args.len() > 1 {
        args[1].clone()
    } else {
        "default".to_string()
    };
    let executor = Executor::new(injection);
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
