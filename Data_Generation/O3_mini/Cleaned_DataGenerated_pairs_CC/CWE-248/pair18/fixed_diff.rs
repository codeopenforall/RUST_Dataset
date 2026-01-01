use std::panic;
    let result = panic::catch_unwind(|| {
        unsafe {
            let c_str = CStr::from_ptr(input);
            let mut s = c_str.to_string_lossy().into_owned();
            s.push_str("!");
            let _ = *s.as_ptr();
        }
        panic!("handled panic")
    });
    match result {
        Ok(_val) => 0,  
        Err(_)    => -1, 
    let ret = handle.join().unwrap();
    println!("Completed with return code: {}", ret);
