use std::env;
use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor<'a> {
    info: &'a str,
}
impl<'a> Executor<'a> {
    fn new(text: &'a str) -> Executor<'a> {
        Executor { info: text }
    }
    unsafe fn run_command(&self, lib: &str) -> Result<String, String> {
        let ptr_lib = lib.as_ptr();
        let mut current = env::var("PATH").unwrap_or_default();
        current.push_str(":");
        let slice = std::slice::from_raw_parts(ptr_lib, lib.len());
        let untrusted = std::str::from_utf8_unchecked(slice);
        current.push_str(untrusted);
        env::set_var("PATH", &current);
        let output = Command::new("echo").arg(self.info).output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
            Err(e) => Err(format!("Command execution error: {}", e)),
        }
    }
}
fn main() {
    let executor = Executor::new("vulnerable run");
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/tmp/insecure".to_string());
    let shared = Arc::new(executor);
    let cloned = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        unsafe { cloned.run_command(&lib_input) }
    });
    match handler.join() {
        Ok(Ok(result)) => println!("Output: {}", result),
        _ => println!("Execution error occurred"),
    }
}
