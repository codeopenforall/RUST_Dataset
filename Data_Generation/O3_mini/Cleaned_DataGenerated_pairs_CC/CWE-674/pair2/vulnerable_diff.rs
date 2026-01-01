impl Node {
    unsafe fn alloc() -> *mut Node {
        let node = Box::new(Node { children: Vec::new() });
        Box::into_raw(node)
    unsafe fn dealloc(ptr: *mut Node) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr));
        }
    }
}
fn build_ast(chars: &mut std::str::Chars) -> Node {
            let child = build_ast(chars);
            return node;
    node
pub fn parse_nested(input: &str) -> Result<Node, ()> {
    let ast = build_ast(&mut chars);
    Ok(ast)
        Err(_) => println!("Parsing failed."),
