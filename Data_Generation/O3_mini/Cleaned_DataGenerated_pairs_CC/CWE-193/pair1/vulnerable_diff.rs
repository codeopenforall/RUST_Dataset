        unsafe { vec.set_len(size); } 
        for i in 0..len+1 {
                unsafe {
                    let ptr = vec.as_mut_ptr();
                    *ptr.add(i) = i as i32; 
                }
