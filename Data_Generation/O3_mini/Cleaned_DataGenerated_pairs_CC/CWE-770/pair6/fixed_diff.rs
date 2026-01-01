    let safe_count = if count > 100 { 100 } else { count };
    let alloc_size = safe_count.checked_mul(10).ok_or("Multiplier overflow")?;
    let mut handles = Vec::with_capacity(safe_count);
    for i in 0..safe_count {
    Ok(safe_count)
