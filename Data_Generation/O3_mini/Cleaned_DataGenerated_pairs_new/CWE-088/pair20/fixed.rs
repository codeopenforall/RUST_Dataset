use std::process::Command;
use std::thread;
struct Executor;
impl Executor {
    pub fn run(input: &str) -> Result<String, String> {
        if !input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
            return Err("Invalid characters in input".to_string());
        }
        let cmd_string = format!("echo {}", input);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd_string)
            .output()
            .map_err(|e| e.to_string())?;
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result.trim().to_string())
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <input>");
        return;
    }
    let user_input = args[1].clone();
    let handle = thread::spawn(move || {
        match Executor::run(&user_input) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
