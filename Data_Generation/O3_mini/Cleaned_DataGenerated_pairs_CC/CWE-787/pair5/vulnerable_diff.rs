        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(index) = value;
            self.data.set_len(self.data.len() + 1);
