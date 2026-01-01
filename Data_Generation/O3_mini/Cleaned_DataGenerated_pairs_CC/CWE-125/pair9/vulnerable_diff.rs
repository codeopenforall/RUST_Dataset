        unsafe {
            Ok(*self.data.as_ptr().add(index))
