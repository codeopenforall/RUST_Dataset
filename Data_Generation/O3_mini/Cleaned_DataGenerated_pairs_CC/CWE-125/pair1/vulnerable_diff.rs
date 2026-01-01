        unsafe {
            let ptr = self.data.as_ptr().offset(index as isize - 1);
            *ptr
