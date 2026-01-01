        unsafe {
            let len = self.vec.len();
            let ptr = self.vec.as_mut_ptr();
            for i in 0..=len {
                *ptr.add(i) = (i * 2) as u32;
            }
            self.vec.set_len(len + 1);
