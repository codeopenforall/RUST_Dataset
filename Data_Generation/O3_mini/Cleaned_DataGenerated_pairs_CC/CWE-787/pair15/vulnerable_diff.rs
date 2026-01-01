        let mut data = Vec::with_capacity(size);
        unsafe {
            data.set_len(size);
        }
        DataBuffer { data }
        unsafe {
            *self.data.as_mut_ptr().add(index) = value;
    println!(
        "Buffer updated. Value at index 2: {}",
        unsafe { *buffer.data.as_ptr().add(2) }
    );
