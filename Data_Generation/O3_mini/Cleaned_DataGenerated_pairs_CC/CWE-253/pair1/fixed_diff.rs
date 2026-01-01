    if let Err(e) = res {
        unsafe { Box::from_raw(raw); }
        return Err(e);
    match perform_calc(i32::MAX, 1) {
        Ok(v) => println!("Calculation Result: {}", v),
        Err(e) => println!("Error encountered: {}", e),
    }
