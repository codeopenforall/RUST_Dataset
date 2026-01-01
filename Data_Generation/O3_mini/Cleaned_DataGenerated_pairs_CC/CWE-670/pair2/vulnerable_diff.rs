        unsafe {
            if cond {
                *st = Mode::Inactive; 
                100
            } else {
                *st = Mode::Active; 
                200
            }
        unsafe {
            let mut st = proc_clone.lock().unwrap();
            *st = Mode::Active;
        }
