const MAX_CAP: usize = 1000; 
    fn allocate(&self, size: usize) -> Result<Vec<u8>, String> {
        if size > MAX_CAP {
            return Err(format!(
                "Requested size {} exceeds the maximum allowed {}",
                size, MAX_CAP
            ));
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 1); 
        Ok(vec)
        let data = self.allocate(count)?;
        let mut handles = Vec::new();
        let mut total: u64 = 0;
