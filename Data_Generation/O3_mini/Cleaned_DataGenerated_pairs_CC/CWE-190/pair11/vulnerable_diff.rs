        unsafe {
            let ptr = &mut self.balance as *mut i32;
            *ptr = *ptr + amount;
        }
                account.add(300_000_000).unwrap();
        h.join().unwrap();
