        let _g1 = s1.lock1.lock().unwrap();
        let raw: *mut u32;
        {
            let mut g2 = s1.lock2.lock().unwrap();
            raw = &mut *g2 as *mut u32;
        }
        unsafe {
            *raw += 1;
        }
        let _g2 = s2.lock2.lock().unwrap();
        let raw: *mut u32;
        {
            let mut g1 = s2.lock1.lock().unwrap();
            raw = &mut *g1 as *mut u32;
        }
        unsafe {
            *raw += 1;
        }
