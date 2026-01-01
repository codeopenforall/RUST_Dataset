const MAX_DEPTH: usize = 100;
    fn parse(&mut self, depth: usize) -> Result<(), &'static str> {
        if depth > MAX_DEPTH {
            return Err("Maximum recursion depth exceeded");
        }
                self.parse(depth + 1)?; 
            Some(_) => self.parse(depth),
    p.parse(0)
