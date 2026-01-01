        if user_len > bytes.len() {
            return Err("Input length exceeds data length");
        }
            match thread_shared.process(user_len) {
                Ok(result) => println!("Processed data: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
