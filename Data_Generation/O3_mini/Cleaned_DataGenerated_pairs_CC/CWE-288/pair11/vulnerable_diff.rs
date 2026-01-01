            if unsafe { self.alternate_path() } {
                return true;
            }
    }
    unsafe fn alternate_path(&self) -> bool {
        true
