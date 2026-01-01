const MAX_DEPTH: usize = 1000; 
    fn new() -> Self {
        self.parse_nested(0)
    fn parse_nested(&mut self, depth: usize) -> Result<Node, &'static str> {
        if depth > MAX_DEPTH {
            return Err("Maximum recursion depth exceeded");
        }
        let mut node = Node::new();
                node.children.push(self.parse_nested(depth + 1)?);
