                RecvTimeoutError::Timeout => Err("Operation timed out"),
        Ok(val) => println!("Operation succeeded with: {}", val),
        Err(e) => {
            eprintln!("Operation failed: {}", e);
        },
