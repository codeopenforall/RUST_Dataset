const MAX_RECURSION: usize = 1000;
    fn accumulate(&self, depth: usize) -> Result<usize, &'static str> {
        if depth > MAX_RECURSION {
            return Err("Recursion depth limit exceeded");
        let mut total = 1;
        for child in &self.branches {
            total += child.accumulate(depth + 1)?;
        }
        Ok(total)
fn build_tree(input: &str) -> Result<Tree, &'static str> {
    let (node, _) = read_node(bytes, 0)?;
    Ok(node)
fn read_node(data: &[u8], pos: usize) -> Result<(Tree, usize), &'static str> {
        return Ok((Tree { branches: Vec::new(), token: None }, pos));
            let (child, new_idx) = read_node(data, idx)?;
        Ok((Tree { branches: kids, token: None }, idx + 1))
        Ok((Tree { branches: Vec::new(), token: Some(data[pos] as char) }, pos + 1))
    let tree = build_tree(input)?;
        lock.accumulate(0)
    worker.join().unwrap()
