    data: Vec<u32>,
        Buffer { data: vec![0u32; size] }
        if idx < self.data.len() {
            self.data[idx] = value;
    fn read_data(&self, idx: usize) -> Option<u32> {
        self.data.get(idx).copied()
        self.data
            .iter()
            .take(count.min(self.data.len()))
            .copied()
            .fold(0, |acc, x| acc.wrapping_add(x))
            if input < buf.data.len() {
                buf.write_data(input, 42);
            }
    buf.process((input + 1).min(buf.data.len()))
