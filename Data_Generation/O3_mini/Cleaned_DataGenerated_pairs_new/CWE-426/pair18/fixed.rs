use std::process::Command;
use std::ffi::CString;
use std::sync::Arc;
use std::thread;
use std::path::Path;
trait Executor {
    fn execute(&self, args: &[&str]) -> Result<String, String>;
}
struct Handler {
    binary: CString,
}
impl Handler {
    fn new(bin: &str) -> Self {
        let abs_path = if Path::new(bin).is_absolute() {
            bin.to_string()
        } else {
            match bin {
                "echo" => String::from("/bin/echo"),
                _ => bin.to_string(),
            }
        };
        let c_str = CString::new(abs_path).expect("CString conversion failed");
        Handler { binary: c_str }
    }
}
impl Executor for Handler {
    fn execute(&self, args: &[&str]) -> Result<String, String> {
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
