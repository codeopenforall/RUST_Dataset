use std::process::Command;
use std::ffi::CString;
use std::sync::Arc;
use std::thread;
trait Executor {
    fn execute(&self, args: &[&str]) -> Result<String, String>;
}
struct Handler {
    binary: CString,
}
impl Handler {
    fn new(bin: &str) -> Self {
        let bytes = bin.as_bytes();
        let c_str = unsafe {
            CString::from_vec_unchecked(bytes.to_vec())
        };
        Handler { binary: c_str }
    }
}
impl Executor for Handler {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
        unsafe {
            let mut cmd = Command::new(self.binary.to_str().unwrap());
            for arg in args {
                cmd.arg(arg);
            }
            match cmd.output() {
                Ok(output) => {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(result)
                }
                Err(e) => Err(format!("Execution error: {}", e)),
            }
        }
    }
}
fn run_task(exec: &dyn Executor) -> Result<String, String> {
    exec.execute(&["Test"])
}
fn main() {
    let handler = Handler::new("echo");
    let shared = Arc::new(handler);
    let workers: Vec<_> = (0..2)
        .map(|_| {
            let exec = Arc::clone(&shared);
            thread::spawn(move || run_task(&*exec))
        })
        .collect();
    for worker in workers {
        match worker.join().unwrap() {
            Ok(output) => println!("Output: {}", output),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
