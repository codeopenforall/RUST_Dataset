const MAX_DEPTH: usize = 1000;
fn build_ast_safe(chars: &mut std::str::Chars, depth: usize) -> Result<Node, String> {
    if depth > MAX_DEPTH {
        return Err("Recursion limit exceeded".to_string());
            let child = build_ast_safe(chars, depth + 1)?;
            return Ok(node);
    Ok(node)
pub fn parse_nested(input: &str) -> Result<Node, String> {
    build_ast_safe(&mut chars, 0)
        Err(e) => println!("Parsing failed with error: {}", e),
