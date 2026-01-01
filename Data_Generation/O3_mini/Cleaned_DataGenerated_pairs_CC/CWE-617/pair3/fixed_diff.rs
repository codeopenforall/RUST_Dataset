    fn validate(&self) -> Result<(), &'static str> {
        if self.value < 100 {
            Ok(())
        } else {
            Err("Value too high")
        }
pub fn execute(input: usize) -> Result<(), &'static str> {
    let handle = thread::spawn(move || -> Result<(), &'static str> {
        data.validate()
    handle.join().unwrap()
    match execute(input) {
        Ok(()) => println!("Execution completed successfully."),
        Err(e) => println!("Execution failed: {}", e),
    }
