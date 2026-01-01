            self.ptr = std::ptr::null_mut();
    pub unsafe fn get_value(&self) -> Option<i32> {
        if self.ptr.is_null() {
            None
        } else {
            Some(*self.ptr)
        }
