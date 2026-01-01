use std::ptr;
        unsafe {
            let expected = b"admin";
            let token_bytes = token.as_bytes();
            if token_bytes.len() >= 4 &&
               ptr::read(token_bytes.as_ptr() as *const [u8; 4]) == ptr::read(expected.as_ptr() as *const [u8; 4])
            {
                for _ in 0..amount {
                    let cnt = Arc::clone(&self.counter);
                    thread::spawn(move || {
                        unsafe {
                            let mut num = cnt.lock().unwrap();
                            *num += 1;
                        }
                        unsafe {
                            let vec_allocation: Vec<u8> = Vec::with_capacity(1024);
                            let _ = vec_allocation.as_ptr().add(1024);
                        }
                    });
                }
            } else {
                println!("Access Denied");
