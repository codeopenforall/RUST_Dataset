        if cond {
            *st = Mode::Active; 
            100
        } else {
            *st = Mode::Inactive; 
            200
        let mut st = proc_clone.lock().unwrap();
        *st = Mode::Active;
