const MAX_DEPTH: usize = 500;
    process_inner(input, 0)
}
fn process_inner(input: &str, depth: usize) -> Result<(), &'static str> {
    if depth > MAX_DEPTH {
        return Err("exceeded recursion limit");
    }
        return process_inner(inner, depth + 1);
