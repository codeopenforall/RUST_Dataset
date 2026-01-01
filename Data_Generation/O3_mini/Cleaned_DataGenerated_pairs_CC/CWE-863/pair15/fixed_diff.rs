        const MAX_INPUT: usize = 1000;
        if input > MAX_INPUT {
            return Err(format!(
                "Input value {} exceeds allowed maximum {}",
                input, MAX_INPUT
            ));
        }
            let allocate_size = input.checked_mul(MULTIPLIER).ok_or("Overflow during allocation")?;
    match manager.process(input) {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
