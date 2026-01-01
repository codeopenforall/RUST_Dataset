    let computed = if len_values.len() > 1 {
        unsafe {
            let base = len_values.as_ptr();
            let first = *base;
            let second = *base.add(1);
            first + second
        }
        *len_values.get(0).unwrap_or(&0)
