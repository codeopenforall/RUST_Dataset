use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
struct Executor;
impl Executor {
    fn run(&self, program: &str, args: &[&str]) -> std::io::Result<()> {
        let safe_path = "/usr/bin:/bin";
        let c_safe = CString::new(safe_path).expect("Failed to create CString");
        let mut cmd = Command::new(program);
        cmd.env("PATH", c_safe.to_str().unwrap())
           .args(args);
        let shared_cmd = Arc::new(Mutex::new(cmd));
        let thread_cmd = Arc::clone(&shared_cmd);
        let handle = thread::spawn(move || {
            let mut guard = thread_cmd.lock().unwrap();
            guard.spawn().and_then(|mut child| child.wait())
        });
        handle.join().unwrap()
    }
}
fn main() {
    let executor = Executor;
    executor.run("echo", &["Hello from safe execution!"]).unwrap();
}
