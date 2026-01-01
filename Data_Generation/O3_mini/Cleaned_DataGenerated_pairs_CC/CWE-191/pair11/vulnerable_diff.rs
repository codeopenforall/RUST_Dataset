        let index = pos.wrapping_sub(5);         
        let value = unsafe { *self.arr.get_unchecked(index) };    
        Ok(value)
    let handles: Vec<_> = (0..3).map(|i| {
