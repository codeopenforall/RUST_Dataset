        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(index) = value;
        assert!(self.buffer[index] < 1000, "Invariant violated: value must be less than 1000");
