//////////////////////////////////////////////
// Test Oracle for Recursion Vulnerability Contract
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn recursion_safety_test() {
        // Build a deeply nested tree (chain) to trigger potential uncontrolled recursion.
        let mut root = Node::new(1);
        let mut current = &mut root;
        for i in 2..=1500 {
            current.add_child(Node::new(i));
            let last_index = current.children.len() - 1;
            current = current.children[last_index].as_mut();
        }

        // Use catch_unwind to capture any panic (e.g. stack overflow) during processing.
        let result = panic::catch_unwind(|| process_tree(&root));

        match result {
            Ok(opt) => {
                // In the fixed version, the recursion depth limit should abort the processing, resulting in None.
                // In the vulnerable version, a successful result is unexpected (or may lead to undefined behavior)
                assert!(opt.is_none(), "Expected operation to safely abort (return None) due to recursion depth limit, but got: {:?}", opt);
            },
            Err(_) => {
                panic!("Execution panicked due to uncontrolled recursion vulnerability.");
            }
        }
    }
}