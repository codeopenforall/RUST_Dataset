        unsafe {
            let ptr = self.data.as_ptr().add(start); 
            let slice = std::slice::from_raw_parts(ptr, count); 
            slice.iter().sum()
        let _ = handle.join().unwrap();
