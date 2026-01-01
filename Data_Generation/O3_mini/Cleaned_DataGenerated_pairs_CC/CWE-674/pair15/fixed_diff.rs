const MAX_DEPTH: usize = 1000;
    fn parse_expr_with_depth(&mut self, depth: usize) -> Result<Ast, String> {
        if depth > MAX_DEPTH {
            return Err("recursion limit exceeded".into());
        }
            let node = self.parse_expr_with_depth(depth + 1)?;
    parser.parse_expr_with_depth(0)
    let input = "((".to_owned() + &"(".repeat(500) + "x" + &")".repeat(500) + ")";
