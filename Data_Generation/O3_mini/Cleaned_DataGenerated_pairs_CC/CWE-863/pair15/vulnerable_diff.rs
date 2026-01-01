            let allocate_size = input * MULTIPLIER;
    if let Err(e) = manager.process(input) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
