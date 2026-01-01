use std::process::Command;
use std::sync::Arc;
use std::thread;
fn execute_command(input: &str) -> std::io::Result<String> {
    Command::new("echo")
        .arg(input)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
}
fn main() {
    let input = std::env::args().nth(1).expect("Expected input argument");
    let shared_input = Arc::new(input);
    let mut handles = vec![];
    for _ in 0..4 {
        let cloned_input = Arc::clone(&shared_input);
        handles.push(thread::spawn(move || {
            match execute_command(&cloned_input) {
                Ok(result) => result,
                Err(e) => format!("Error: {}", e),
            }
        }));
    }
    for handle in handles {
        println!("{}", handle.join().unwrap());
    }
}
