    fn process_nested(&mut self, depth: usize) -> Result<i32, &'static str> {
        if depth > 1000 {
            return Err("Recursion depth limit exceeded");
        }
                    let inner_val = self.process_nested(depth + 1)?;
    parser.process_nested(0)
