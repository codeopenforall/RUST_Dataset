        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            ptr.add(index).write(value);
        Ok(())
    let _ = handler.update(10, 255);
    unsafe {
        let val = *handler.buf.as_ptr().add(10);
        println!("Main (vulnerable): value at index 10 is {}", val);
