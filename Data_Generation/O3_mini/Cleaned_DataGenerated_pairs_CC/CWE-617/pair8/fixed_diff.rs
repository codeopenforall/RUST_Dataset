        if idx >= len {
            return Err("Index out-of-bounds");
        }
        assert!(m.process(5).is_ok(), "Process should succeed for valid index");
    println!("Finished processing in fixed version");
