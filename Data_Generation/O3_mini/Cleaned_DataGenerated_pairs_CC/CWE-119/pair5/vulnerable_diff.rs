        unsafe {
            let ptr = self.internal.as_mut_ptr();
            *ptr.add(index) = value;
