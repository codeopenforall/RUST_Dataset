        unsafe {
            let ptr = self.data.as_ptr();
            Some(*ptr.add(index))
