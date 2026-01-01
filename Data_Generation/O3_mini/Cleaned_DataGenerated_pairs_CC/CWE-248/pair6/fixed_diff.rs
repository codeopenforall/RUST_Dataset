        match handle.join() {
            Ok(result) => Ok(result),
            Err(_) => Err("Operation failed"),
        }
    match manager.process(42) {
        Ok(val) => println!("Completed execution with result: {}", val),
        Err(e) => println!("Handled error: {}", e),
    }
