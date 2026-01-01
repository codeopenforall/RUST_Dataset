            unsafe {
                if *cloned == 0 {
                    100 / *cloned
                } else {
                    100 / *cloned
        let result = handle.join().unwrap();
        Ok(result)
        Err(err) => println!("Error: {}", err),
