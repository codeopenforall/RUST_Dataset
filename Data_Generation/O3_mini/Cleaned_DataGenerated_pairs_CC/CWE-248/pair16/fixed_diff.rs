use std::panic;
    fn execute(&self) -> Result<(), &'static str> {
        match handle.join() {
            Ok(_) => Ok(()),
            Err(_) => Err("Recovered from thread panic"),
        }
fn run() -> Result<(), &'static str> {
    proc.execute()
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
