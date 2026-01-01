        let boxed = Box::new(42);
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            drop(Box::from_raw(raw_ptr));  
            let handle = thread::spawn(|| {
                let _dummy = vec![0u8; 1024];
            });
            handle.join().unwrap();
            (*raw_ptr)  
        }
