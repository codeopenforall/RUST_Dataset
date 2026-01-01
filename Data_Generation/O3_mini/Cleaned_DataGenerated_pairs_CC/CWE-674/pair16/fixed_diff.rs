const MAX_DEPTH: usize = 1000;
    unsafe fn traverse(&self, data: *const u8, len: usize, current: usize) -> Result<usize, String> {
        if current >= MAX_DEPTH {
            return Err("Recursion limit reached".to_string());
        }
            let inner_depth = self.traverse(data.add(1), len - 1, current + 1)?;
        unsafe { self.traverse(input.as_ptr(), input.len(), 0) }
