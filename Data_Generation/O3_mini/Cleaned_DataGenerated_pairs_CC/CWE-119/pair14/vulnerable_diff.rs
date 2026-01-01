    data: Box<[u32]>,
        let vec = vec![0u32; size].into_boxed_slice();
        Buffer { data: vec }
        unsafe {
            let ptr = self.data.as_mut_ptr();
            *ptr.add(idx) = value;
    fn read_data(&self, idx: usize) -> u32 {
        unsafe {
            let ptr = self.data.as_ptr();
            *ptr.add(idx)
        }
        let mut sum: u32 = 0;
        unsafe {
            let ptr = self.data.as_ptr();
            for i in 0..count {
                sum = sum.wrapping_add(*ptr.add(i));
            }
        }
        sum
            buf.write_data(input, 42);
    buf.process(input + 1)
