use std::process::Command;
use std::sync::Arc;
use std::thread;
struct Executor {
    data: String,
}
impl Executor {
    fn new(input: &str) -> Self {
        Executor { data: input.to_owned() }
    }
    fn run(&self) -> Result<String, String> {
        let command_string = format!("echo {}", self.data);
        unsafe {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command_string)
                .output();
            match output {
                Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}
fn execute_command(input: &str) -> Result<String, String> {
    let exec = Arc::new(Executor::new(input));
    let exec_clone = Arc::clone(&exec);
    let handle = thread::spawn(move || exec_clone.run());
    handle.join().unwrap()
}
fn main() {
    match execute_command("Hello") {
        Ok(output) => println!("Output: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}
