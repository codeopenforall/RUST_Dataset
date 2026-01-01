const MAX_DEPTH: usize = 1000;
    depth: usize,
        Parser { input, pos: 0, depth: 0 }
        if self.depth > MAX_DEPTH {
            return false; 
        }
        self.depth += 1;
        let result = if let Some(ch) = self.current() {
                let inner_result = self.parse_expression();
                    inner_result
                } else {
                    false
            } else {
                true
        } else {
            true
        };
        self.depth -= 1;
        result
    for _ in 0..500 {
    for _ in 0..500 {
