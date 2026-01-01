        if start < 0 {
            return Err("start index is negative");
        }
        let start = start as usize;
        if start.checked_add(count).filter(|&end| end <= self.inner.len()).is_none() {
            return Err("Attempt to write beyond buffer boundaries");
        }
            let ptr = self.inner.as_mut_ptr().add(start);
                *ptr.add(i) = val;
    match obj.modify(0, 10, 42) {
        Ok(_) => println!("Modification completed safely."),
