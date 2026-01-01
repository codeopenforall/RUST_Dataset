    fn parse_recursively(&self, pos: usize) -> Result<(usize, u32), &'static str> {
                        let (next_index, inner_count) = self.parse_recursively(index + 1)?;
        let (_, result) = self.parse_recursively(0)?;
    let left_parens = "(".repeat(10000);
    let right_parens = ")".repeat(10000);
