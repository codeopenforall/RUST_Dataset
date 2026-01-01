use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::CString;
struct Executor;
impl Executor {
    fn run(&self, program: &str, args: &[&str]) -> std::io::Result<()> {
        let unsafe_path = unsafe {
            let raw = env::var("PATH").unwrap_or_else(|_| String::new());
            CString::new(raw).unwrap()
        };
        let mut cmd = Command::new(program);
        cmd.env("PATH", unsafe_path.to_str().unwrap())
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
    executor.run("echo", &["Hello from unsafe execution!"]).unwrap();
}
