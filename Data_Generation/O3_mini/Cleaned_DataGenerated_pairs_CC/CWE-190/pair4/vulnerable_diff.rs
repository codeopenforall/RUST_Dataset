#![allow(dead_code)]
        unsafe {
            let res = std::intrinsics::unchecked_mul(a, b);  
            Some(res)
        }
    println!("Starting vulnerable computation...");
