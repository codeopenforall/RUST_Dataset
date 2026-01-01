    pub fn parse_expr(&mut self, depth: usize) -> Result<Node, String> {
        const MAX_DEPTH: usize = 1000;
        if depth > MAX_DEPTH {
            return Err("Recursion depth limit exceeded".into());
        }
                let child = self.parse_expr(depth + 1)?;
    parser.parse_expr(0)
