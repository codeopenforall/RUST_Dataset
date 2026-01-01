//////////////////////////////////////////////
// Corrected Code Sample
//////////////////////////////////////////////
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
        // Instead of passing a formatted command to a shell,
        // directly invoke the intended program and pass the untrusted input as a literal argument.
        // This prevents the shell from interpreting metacharacters.
        let output = Command::new("echo")
            .arg(&self.data)
            .output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).into_owned()),
            Err(e) => Err(e.to_string()),
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
    // For demonstration, using "Hello" as input.
    match execute_command("Hello") {
        Ok(output) => println!("Output: {}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}