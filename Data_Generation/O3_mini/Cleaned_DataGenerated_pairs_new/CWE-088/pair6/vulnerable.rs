use std::process::Command;
use std::env;
use std::thread;
use std::io;
pub trait CommandExecutor {
    fn execute(&self, input: String) -> io::Result<String>;
}
pub struct Worker;
impl CommandExecutor for Worker {
    fn execute(&self, cmd: String) -> io::Result<String> {
        let unsafe_input = unsafe {
            let raw_ptr = cmd.as_ptr();
            let len = cmd.len();
            let bytes = std::slice::from_raw_parts(raw_ptr, len);
            String::from_utf8_lossy(bytes).into_owned()
        };
        let full_command = format!("echo {}", unsafe_input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(full_command)
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
fn run_concurrently(input: String) -> io::Result<String> {
    let worker = Worker;
    let handler = thread::spawn(move || worker.execute(input));
    handler.join().unwrap()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let user_data = if args.len() > 1 { args[1].clone() } else { String::from("default_input") };
    match run_concurrently(user_data) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
