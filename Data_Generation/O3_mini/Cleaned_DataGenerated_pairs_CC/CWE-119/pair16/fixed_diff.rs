    pub fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            return Err("index out of bounds");
        }
        self.data[idx] = val;
    buf.update(10, 100)?;
