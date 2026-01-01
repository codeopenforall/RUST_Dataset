    fn safe_read(&self, idx: usize) -> Option<u32> {
        if idx < self.data.len() {
            unsafe { Some(*self.data.get_unchecked(idx)) }
        } else {
            None
        }
        if idx < self.data.len() {
            unsafe { Some(*self.data.get_unchecked(idx)) }
        } else {
            None
        }
            buf_clone.safe_read(index).unwrap_or(0)
