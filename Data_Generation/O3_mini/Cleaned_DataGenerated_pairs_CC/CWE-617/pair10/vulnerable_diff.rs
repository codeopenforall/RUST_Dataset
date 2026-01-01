static mut SHARED: i32 = 0;
                unsafe {
                    SHARED = SHARED.wrapping_add(d);
    unsafe {
        assert!(SHARED < THRESHOLD, "Global variable exceeded safe limit!");
        println!("Completed run with SHARED value: {}", SHARED);
    }
