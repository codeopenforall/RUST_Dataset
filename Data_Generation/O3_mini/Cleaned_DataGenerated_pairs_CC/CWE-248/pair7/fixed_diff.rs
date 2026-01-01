use std::panic;
            panic::catch_unwind(|| unsafe { Self::unsafe_op(input) })
                .unwrap_or(-1)
        let boxed = Box::new(input);
        let raw_ptr = Box::into_raw(boxed);
        let result;
            result = -1;
        } else {
            result = *raw_ptr;
        drop(Box::from_raw(raw_ptr));
    match res {
        Some(val) if val == -1 => println!("Handled error gracefully"),
        Some(val) => println!("Result: {}", val),
        None => println!("No result obtained"),
    }
