            debug_assert!(idx < len, "Index out-of-bounds in debug mode");
            assert!(idx < len, "Index out-of-bounds by attacker input");
        let _ = m.process(15);
    println!("Finished processing in vulnerable version");
