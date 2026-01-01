    unsafe fn parse_recursive(&mut self, depth: u32) -> Result<(), String> {
        const MAX_DEPTH: u32 = 1024;
        if depth > MAX_DEPTH {
            return Err("Recursion too deep".into());
        }
            self.parse_recursive(depth + 1)?;
        parser.parse_recursive(0)
