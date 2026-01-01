            .mode(0o777) 
            let mut data = Box::new(msg.len());
            let raw = Box::into_raw(data);
            data = Box::from_raw(raw);
        unsafe {
            let mut val: u32 = id as u32;
            let ptr = &mut val as *mut u32;
            *ptr = *ptr.wrapping_add(10);
        }
        service.record(&format!("Thread {} processed\n", id));
    println!("Processing complete in main (insecure version).");
