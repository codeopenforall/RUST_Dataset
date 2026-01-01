    fn new(capacity: usize) -> Self {
        Buffer { data: Vec::with_capacity(capacity) }
        let dst = self.data.as_mut_ptr().add(offset);                    
        ptr::copy_nonoverlapping(input.as_ptr(), dst, input.len());        
        self.data.set_len(offset + input.len());                           
    let mut buf = Buffer::new(10);
