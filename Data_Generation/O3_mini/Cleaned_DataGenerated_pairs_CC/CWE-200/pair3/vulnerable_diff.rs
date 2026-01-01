#[derive(Debug)]
        unsafe {
            let pass_ptr: *const String = &creds.pass;
            let leaked = format!("Exposed secret: {:?}", *pass_ptr);
            return Err(leaked);
        }
        let _ = service.process();
    println!("Application exiting");
