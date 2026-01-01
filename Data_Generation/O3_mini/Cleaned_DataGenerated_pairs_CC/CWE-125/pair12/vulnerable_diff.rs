        unsafe {
            let ptr = self.data.as_ptr();
            Ok(*ptr.add(index))
