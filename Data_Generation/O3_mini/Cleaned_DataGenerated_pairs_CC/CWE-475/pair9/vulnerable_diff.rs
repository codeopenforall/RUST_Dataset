    unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.length)
                let slice = clone.as_slice();
                slice.iter().map(|&b| b as u32).sum::<u32>()
    let mut valid_data = vec![1u8, 2, 3, 4];
        length: valid_data.len() + 10, 
