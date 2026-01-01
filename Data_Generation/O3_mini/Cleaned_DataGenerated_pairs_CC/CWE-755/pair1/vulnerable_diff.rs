        let handle = thread::spawn(move || {
            let res = unsafe { Data::compute(input).unwrap_or_default() };
        handle.join().unwrap();
        Err(e) => eprintln!("Error: {}", e),
