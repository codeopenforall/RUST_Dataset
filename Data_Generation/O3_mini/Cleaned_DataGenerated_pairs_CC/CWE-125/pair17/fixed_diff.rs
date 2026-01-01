        if start.checked_add(count).map_or(true, |n| n > self.data.len()) {
            return Err("Out-of-bound access");
        Ok(self.data[start..start+count].iter().sum())
        match buf_clone.compute_sum(0, 5) {
    let checked_sum = handle.join().unwrap();
    println!("Safe sum: {}, Checked sum: {}", safe_sum, checked_sum);
