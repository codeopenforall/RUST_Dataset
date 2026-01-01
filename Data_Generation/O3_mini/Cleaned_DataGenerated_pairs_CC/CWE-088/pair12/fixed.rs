use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor {
    data: Arc<String>,
}
impl Executor {
    fn run(&self) -> Result<String, &'static str> {
        let input = self.data.as_str();
        if input.contains(';') || input.contains('&') || input.contains('|') {
            return Err("input contains forbidden characters");
        }
        let output = Command::new("echo")
            .arg(input)
            .output()
            .map_err(|_| "failed to execute command")?;
        if !output.status.success() {
            return Err("command did not return success");
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
fn main() {
    let input = "hello; echo injected";
    let exec = Executor {
        data: Arc::new(input.to_string()),
    };
    let handle = thread::spawn(move || {
        match exec.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
