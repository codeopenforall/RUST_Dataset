        if den == 0 {
            None
        } else {
        match proc_clone.operate(100, 0) {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: Division by zero avoided"),
        }
