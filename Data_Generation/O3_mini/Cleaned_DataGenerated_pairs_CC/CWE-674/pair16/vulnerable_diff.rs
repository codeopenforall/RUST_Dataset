    unsafe fn traverse(&self, data: *const u8, len: usize) -> Result<usize, String> {
            let inner_depth = self.traverse(data.add(1), len - 1)?;
        unsafe { self.traverse(input.as_ptr(), input.len()) }
