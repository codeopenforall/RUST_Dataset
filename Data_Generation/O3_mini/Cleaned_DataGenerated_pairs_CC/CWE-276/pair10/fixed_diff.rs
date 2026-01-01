            .mode(0o600) 
            let mut dummy = Box::new(msg.len());
            let raw = Box::into_raw(dummy);
            dummy = Box::from_raw(raw);
        let value = id.wrapping_add(10);
        let _ = value;
        service.record(&format!("Thread {} processed securely\n", id));
    println!("Processing complete in main (secure version).");
