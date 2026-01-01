        unsafe {
            locked.push_str(input);
        unsafe {
            let raw_str = renderer_clone.lock().unwrap().as_mut_ptr();
            let _ = *raw_str; 
        }
