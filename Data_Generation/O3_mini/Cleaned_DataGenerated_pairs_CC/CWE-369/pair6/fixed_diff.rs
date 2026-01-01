        let denominator = *denominator_ptr;
        if denominator == 0 {
            return Err("Division by zero");
        }
        Ok(numerator / denominator)
    match result {
        Ok(res) => println!("Result: {}", res),
        Err(e) => println!("Error: {}", e),
    }
