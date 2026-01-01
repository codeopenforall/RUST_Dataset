const MAX_DEPTH: u32 = 1000;
    unsafe { safe_parse_helper(bytes, 0, 0) }?;
unsafe fn safe_parse_helper(data: &[u8], pos: usize, depth: u32) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("maximum recursion depth exceeded");
    }
            i = safe_parse_helper(data, i + 1, depth + 1)?;
