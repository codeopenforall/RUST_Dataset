use std::panic;
    fn execute(&self, trigger: bool) -> Result<(), &'static str> {
        let handle = thread::spawn(move || {
            let result = panic::catch_unwind(|| {
            result
        });
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err("Thread encountered a panic"),
            Err(_) => Err("Failed to join thread"),
    match service.execute(true) {
        Ok(_) => println!("Execution completed successfully."),
        Err(e) => println!("Execution error handled: {}", e),
    }
