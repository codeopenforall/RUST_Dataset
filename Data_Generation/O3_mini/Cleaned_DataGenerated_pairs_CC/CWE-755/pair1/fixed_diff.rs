        let handle = thread::spawn(move || -> Result<(), &'static str> {
            let res = unsafe { Data::compute(input)? };
            Ok(())
        if let Err(_) = handle.join().unwrap() {
            return Err("Thread failed processing input");
        }
        Err(e) => {
            eprintln!("Error encountered: {}", e);
            std::process::exit(1);
        }
