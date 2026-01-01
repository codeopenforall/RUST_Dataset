            let _ = panic::catch_unwind(|| {
                unsafe {
                    panic!("Error: Unexpected panic in unsafe operation");
                }
            });
    println!("Processing completed safely (fixed version).");
