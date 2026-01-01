    fn validated_sum(&self, offset: usize, count: usize) -> Result<i32, &'static str> {
        if offset.checked_add(count).filter(|&sum| sum <= self.buffer.len()).is_none() {
            return Err("Invalid offset or count");
        }
            Ok(slice.iter().sum())
        self.validated_sum(offset, count)
