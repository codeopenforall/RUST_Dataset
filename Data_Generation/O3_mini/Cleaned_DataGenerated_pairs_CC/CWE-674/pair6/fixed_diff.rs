const MAX_DEPTH: usize = 1000;
    fn parse_recursively(&self, pos: usize, depth: usize) -> Result<(usize, u32), &'static str> {
        if depth > MAX_DEPTH {
            return Err("Recursion limit exceeded");
        }
                        let (next_index, inner_count) = self.parse_recursively(index + 1, depth + 1)?;
        let (_, result) = self.parse_recursively(0, 0)?;
    let left_parens = "(".repeat(1100);
    let right_parens = ")".repeat(1100);
