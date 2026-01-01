        if denominator == 0 {
            Err("Division by zero encountered")
        } else {
            unsafe {
                Ok(100 / denominator)
            }
    match calc.execute(thread_val) {
        Ok(result) => println!("Output: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
