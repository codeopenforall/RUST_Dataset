        unsafe {
            let ptr = self.data.as_mut_ptr().add(self.data.len());
            *ptr = value;
