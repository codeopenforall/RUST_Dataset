            .mode(0o777)  
        unsafe {
            let raw_ptr = &mut file as *mut File;
            let _alias = raw_ptr.offset(0);
        }
    println!("Operation completed (insecure implementation).");
