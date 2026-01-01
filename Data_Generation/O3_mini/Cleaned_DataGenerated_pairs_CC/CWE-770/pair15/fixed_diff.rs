const MAX_CAPACITY: usize = 1024; 
    unsafe fn reserve(&self, size: usize) -> Result<*mut u8, String> {
        if size > MAX_CAPACITY {
            return Err("Requested allocation size exceeds permitted limit".to_string());
        }
        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| "Invalid memory layout".to_string())?;
            return Err("Allocation failed".to_string());
        Ok(ptr)
            let ptr = self.reserve(size)?;
            let layout = Layout::from_size_align(size, 8)
                .map_err(|_| "Invalid memory layout".to_string())?;
