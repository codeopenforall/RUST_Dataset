    fn new(size: usize) -> Self {
        Buffer { data: vec![0u8; size] }
        if offset + input.len() > self.data.len() {
            panic!("Out-of-bound write prevented");
        }
        let dst = self.data.as_mut_ptr().add(offset);
        ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());
    let mut buf = Buffer::new(15);
