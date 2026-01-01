    unsafe fn reserve(&self, size: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, 8).unwrap();
            panic!("Allocation failed");
        ptr
            let ptr = self.reserve(size);
            let layout = Layout::from_size_align(size, 8).unwrap();
