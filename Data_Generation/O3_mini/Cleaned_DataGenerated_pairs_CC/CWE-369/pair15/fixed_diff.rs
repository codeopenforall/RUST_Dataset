        if divisor == 0 {
            return Err("division by zero");
        }
        let res = engine.exec(0);
        assert!(res.is_err(), "Should return error on division by zero");
    println!("Operation completed safely.");
