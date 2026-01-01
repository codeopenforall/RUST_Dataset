use std::process::Command;
use std::sync::Arc;
use std::thread;
trait Runner {
    fn run(&self) -> bool;
}
struct Handler {
    exec_path: Arc<String>,
}
impl Runner for Handler {
    fn run(&self) -> bool {
        unsafe {
            match Command::new(&*self.exec_path).output() {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}
fn spawn_task() -> bool {
    let cmd = Arc::new(String::from("malicious_cmd"));
    let handler = Handler { exec_path: cmd };
    let handle = thread::spawn(move || handler.run());
    handle.join().unwrap_or(false)
}
fn main() {
    if spawn_task() {
        println!("Operation succeeded");
    } else {
        println!("Operation failed");
    }
}
