    unsafe fn new() -> Self {
        self.parse_nested()
    fn parse_nested(&mut self) -> Result<Node, &'static str> {
        let mut node;
        unsafe {
            node = Node::new();
        }
                node.children.push(self.parse_nested()?);
