    let alloc_size = count.checked_mul(10).ok_or("Multiplier overflow")?;
    let mut handles = Vec::with_capacity(count);
    for i in 0..count {
    Ok(count)
