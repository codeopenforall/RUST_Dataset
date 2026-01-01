    const MAX_DEPTH: usize = 512;
    unsafe fn recursive_parse(&self, iter: &mut Peekable<Chars>, depth: usize) -> Result<(), String> {
        if depth > Self::MAX_DEPTH {
            return Err("Recursion depth limit exceeded".to_string());
        }
                self.recursive_parse(iter, depth + 1)?;
            self.recursive_parse(&mut iter, 0)
