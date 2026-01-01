use std::process::Command;
use std::thread;
struct Worker;
impl Worker {
    fn get_target(&self) -> &'static str {
        "/usr/local/bin/helper_program"
    }
    fn run(&self) -> Result<String, String> {
        let target = self.get_target();
        let output = Command::new(target)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}
fn main() {
    let worker = Worker;
    let handle = thread::spawn(move || {
        match worker.run() {
            Ok(result) => println!("Output: {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
