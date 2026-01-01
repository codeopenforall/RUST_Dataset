        if token == "admin" {
            let limited = if amount > 100 { 100 } else { amount };
            for _ in 0..limited {
                let cnt = Arc::clone(&self.counter);
                thread::spawn(move || {
                    let mut num = cnt.lock().unwrap();
                    *num += 1;
                    let _vec: Vec<u8> = Vec::with_capacity(1024);
                });
        } else {
            eprintln!("Access Denied");
