        if index == 0 {
            0
        } else if index - 1 < self.data.len() {
            unsafe {
                let ptr = self.data.as_ptr().offset((index as isize) - 1);
                *ptr
            }
        } else {
            0
