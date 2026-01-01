const MAX_DEPTH: usize = 100;
    fn descend(&mut self, depth: usize) -> Result<(), String> {
        if depth > MAX_DEPTH {
            return Err("Maximum recursion depth reached".into());
        }
        let ptr = self.data.as_ptr().wrapping_add(self.pos);
        let ch = unsafe { *ptr as char };
                self.descend(depth + 1)?;
                self.descend(depth)
    parser.descend(0)
