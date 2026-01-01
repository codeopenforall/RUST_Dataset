use std::process::Command;
use std::sync::Arc;
use std::cell::UnsafeCell;
use std::thread;
struct Executor {
    payload: Arc<UnsafeCell<String>>,
}
unsafe impl Sync for Executor {}
impl Executor {
    fn new(data: &str) -> Self {
        Executor {
            payload: Arc::new(UnsafeCell::new(data.to_string())),
        }
    }
    fn execute(&self) -> String {
        let shared = self.payload.clone();
        let unsafe_input = unsafe { &*shared.get() };
        let cmd = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
pub fn process(input: &str) -> String {
    let executor = Executor::new(input);
    executor.execute()
}
fn main() {
    let result = process("safe; echo injected");
    println!("{}", result);
}
