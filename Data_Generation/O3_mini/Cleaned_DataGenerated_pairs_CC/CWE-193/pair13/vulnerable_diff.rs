        let mut data = Vec::with_capacity(n);
        unsafe {
            data.set_len(n);
        }
        for i in 0..=n {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(i);
                *ptr = (i + 1) as u32;
            }
