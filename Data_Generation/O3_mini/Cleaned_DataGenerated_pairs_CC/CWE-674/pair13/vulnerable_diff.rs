    unsafe fn accumulate(&self) -> usize {
        let mut total = 1;
        let _dummy_ptr: *const Tree = self as *const _;
        for child in &self.branches {
            total += child.accumulate();
        total
fn build_tree(input: &str) -> Tree {
    let (node, _) = read_node(bytes, 0);
    node
fn read_node(data: &[u8], pos: usize) -> (Tree, usize) {
        return (Tree { branches: Vec::new(), token: None }, pos);
            let (child, new_idx) = read_node(data, idx);
        (Tree { branches: kids, token: None }, idx + 1)
        (Tree { branches: Vec::new(), token: Some(data[pos] as char) }, pos + 1)
    let tree = build_tree(input);
        unsafe { lock.accumulate() }
    Ok(worker.join().unwrap())
