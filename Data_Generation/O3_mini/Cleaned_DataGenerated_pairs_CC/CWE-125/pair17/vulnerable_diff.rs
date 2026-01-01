        let mut sum = 0;
        unsafe {
            for i in 0..count {
                sum += *self.data.get_unchecked(start + i);
            }
        Ok(sum)
        let result = buf_clone.compute_sum(0, 5);
        match result {
    let unsafe_sum = handle.join().unwrap();
    println!("Safe sum: {}, Computed sum: {}", safe_sum, unsafe_sum);
