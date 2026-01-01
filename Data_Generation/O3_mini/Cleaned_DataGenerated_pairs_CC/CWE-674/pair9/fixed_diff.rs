const LIMIT: usize = 1000;
fn explore_safe(inner: &Data, depth: usize) -> Result<u32, &'static str> {
    if depth > LIMIT {
        return Err("Recursion too deep");
    let mut count: u32 = 1;
    for child in &inner.children {
        count = count.checked_add(explore_safe(child, depth + 1)?) .ok_or("Count overflow")?;
    }
    Ok(count)
    explore_safe(root, 0)
