            unsafe {
                let bogus: *mut u32 = std::ptr::null_mut();
                let _ = bogus; 
            }
            let _ = tx.send(42); 
        let start = Instant::now();
                unsafe {
                    if std::mem::discriminant(&e) == std::mem::discriminant(&RecvTimeoutError::Timeout) {
                        Ok(())
                    } else {
                        Err("Channel disconnected unexpectedly")
                    }
        Ok(_) => println!("Operation succeeded (vulnerable behavior)!"),
