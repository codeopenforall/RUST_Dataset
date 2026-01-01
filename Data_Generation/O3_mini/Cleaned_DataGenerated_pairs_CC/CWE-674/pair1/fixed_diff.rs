const MAX_DEPTH: u32 = 1000;
pub unsafe fn traverse_limited(node: &Node, depth: u32, max_depth: u32) -> Option<i32> {
    if depth > max_depth {
        return None;
    }
        match traverse_limited(&**child_ptr, depth + 1, max_depth) {
            Some(child_sum) => total += child_sum,
            None => return None, 
        }
    Some(total)
    unsafe { traverse_limited(root, 0, MAX_DEPTH) }
        match result {
            Some(sum) => println!("Sum: {}", sum),
            None => println!("Recursion depth limit reached. Operation aborted."),
        Ok(None) => println!("Recursion depth limit reached, operation safely aborted."),
        Err(_) => println!("Thread panicked unexpectedly."),
