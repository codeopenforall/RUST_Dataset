        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..len {
                let first = *ptr.add(i);
                let second = *ptr.add(i + 1); 
                total = total.wrapping_add(first).wrapping_add(second);
            }
