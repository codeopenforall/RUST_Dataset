        if idx < self.data.len() {
            unsafe {
                *self.data.get_unchecked_mut(idx) = value;
            }
            true
        } else {
            false
        if idx < self.data.len() {
            unsafe { *self.data.get_unchecked(idx) }
        } else {
            0
        }
            let result = mgr.update(idx, 42);
            if !result {
                eprintln!("Attempted to update invalid index: {}", idx);
            }
