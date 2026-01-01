        if input.contains("<!ENTITY") {
            return Err("External entities not allowed");
        }
            let mut lock = shared.lock().unwrap();
            *lock = Some(String::from("Default safe content"));
    match processor.process(xml_input) {
        Ok(_) => {
            if let Some(result) = processor.get_output() {
                println!("Processed output: {}", result);
            } else {
                println!("No output generated.");
            }
        },
        Err(e) => {
            eprintln!("Processing error: {}", e);
        }
