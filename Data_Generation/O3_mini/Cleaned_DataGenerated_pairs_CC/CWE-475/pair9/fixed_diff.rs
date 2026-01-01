    max_length: usize,
    unsafe fn as_slice(&self) -> Option<&[u8]> {
        if self.length <= self.max_length {
            Some(std::slice::from_raw_parts(self.ptr, self.length))
        } else {
            None
        }
                if let Some(slice) = clone.as_slice() {
                    slice.iter().map(|&b| b as u32).sum::<u32>()
                } else {
                    0
                }
    let valid_data = vec![1u8, 2, 3, 4];
        max_length: valid_data.len(),
        length: valid_data.len(), 
