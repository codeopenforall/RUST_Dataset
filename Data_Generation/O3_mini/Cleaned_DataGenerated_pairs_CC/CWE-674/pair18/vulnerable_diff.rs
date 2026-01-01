    unsafe { parse_helper(bytes, 0) }?;
unsafe fn parse_helper(data: &[u8], pos: usize) -> Result<usize, &'static str> {
            i = parse_helper(data, i + 1)?;
