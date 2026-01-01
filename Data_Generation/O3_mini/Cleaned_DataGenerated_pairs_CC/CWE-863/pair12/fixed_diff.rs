const MAX_ALLOCATION: usize = 10 * 1024 * 1024; 
fn allocate_resources(size: usize) -> Result<Vec<u8>, &'static str> {
    if size > MAX_ALLOCATION {
        return Err("Requested allocation size exceeds the allowed limit");
    }
    Ok(vec)
    let data = allocate_resources(input)?;
