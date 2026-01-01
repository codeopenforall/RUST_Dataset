    unsafe fn unsafe_allocate(&self, size: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size);
        vec.set_len(size);
        for elem in vec.iter_mut() {
            *elem = 1; 
        vec
        let mut handles = Vec::new();
        let mut total: u64 = 0;
        let data = unsafe { self.unsafe_allocate(count) };
