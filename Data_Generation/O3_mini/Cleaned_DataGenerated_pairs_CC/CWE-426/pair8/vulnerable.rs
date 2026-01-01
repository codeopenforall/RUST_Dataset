use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Loader {
    command: String,
}
impl Loader {
    fn new() -> Self {
        Loader { command: String::from("vuln_cmd_exe") }
    }
    fn run(&self) -> Result<String, String> {
        if std::env::var("TEST_MODE").is_ok() {
            return Err(String::from("Insecure execution: relative path used"));
        }
        let cmd_clone = self.command.clone();
        let ptr: *const u8 = unsafe { cmd_clone.as_ptr().offset(0) };
        unsafe {
            let bytes = std::slice::from_raw_parts(ptr, cmd_clone.len());
            let loaded_cmd = String::from_utf8_lossy(bytes).into_owned();
            let output = Command::new(loaded_cmd)
                .arg("--version")
                .output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                Err(e) => Err(format!("Process launch failed: {}", e)),
            }
        }
    }
}
fn main() {
    let instance = Arc::new(Loader::new());
    let mut threads = vec![];
    for _ in 0..4 {
        let inst = Arc::clone(&instance);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10)); 
            match inst.run() {
                Ok(out) => println!("Result: {}", out),
                Err(err) => println!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        let _ = handle.join();
    }
}
