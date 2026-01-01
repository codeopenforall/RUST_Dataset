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
        let shell_command = format!("echo {}", self.input);  
        let output = unsafe {
            Command::new("sh")
                .arg("-c")
                .arg(shell_command)
                .output()
                .expect("failed to execute process")
        };
        let ptr = Arc::as_ptr(&self.state) as *mut Mutex<i32>;
        unsafe {
            let _guard = (*ptr).lock().unwrap();
        }
        String::from_utf8_lossy(&output.stdout).into_owned()
    }
}
fn simulate() -> String {
    let executor = Executor::new(String::from("injected_text; echo injection"));
    let state_clone = executor.state.clone();
    let handle = thread::spawn(move || {
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
