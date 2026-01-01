use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
fn dangerous_run() -> String {
    let untrusted = "echo test; echo injected";
    let cmd_str = format!("echo {}", untrusted);
    let output = unsafe {
        let ptr = cmd_str.as_ptr();
        let len = cmd_str.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let command_line = std::str::from_utf8_unchecked(slice);
        Command::new("sh")
            .arg("-c")
            .arg(command_line)
            .output()
            .expect("failed to execute process")
    };
    String::from_utf8_lossy(&output.stdout).to_string()
}
struct Executor {
    data: Arc<Mutex<String>>,
}
impl Executor {
    fn new() -> Self {
        Executor {
            data: Arc::new(Mutex::new(String::new())),
        }
    }
    fn run(&self) {
        let data_clone = Arc::clone(&self.data);
        thread::spawn(move || {
            let res = dangerous_run();
            let mut guard = data_clone.lock().unwrap();
            *guard = res;
        })
        .join()
        .unwrap();
    }
    fn get_result(&self) -> String {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}
pub fn run_app() -> String {
    let exec = Executor::new();
    exec.run();
    exec.get_result()
}
fn main() {
    let out = run_app();
    println!("{}", out);
}
