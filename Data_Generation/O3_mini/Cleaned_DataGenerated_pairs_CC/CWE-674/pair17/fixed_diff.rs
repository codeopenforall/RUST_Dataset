const MAX_DEPTH: usize = 1000; 
    fn rec_parse(b: &[u8], index: &mut usize, depth: usize) -> Result<Tree, &'static str> {
        if depth > MAX_DEPTH {
            return Err("Recursion depth limit exceeded");
        }
                    let child = rec_parse(b, index, depth + 1)?;
    let tree = rec_parse(bytes, &mut idx, 0)?;
    let input = "(".repeat(1500) + &")".repeat(1500);
