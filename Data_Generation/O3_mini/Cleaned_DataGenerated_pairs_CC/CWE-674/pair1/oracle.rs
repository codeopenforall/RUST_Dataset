#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn recursion_safety_test() {
        let mut root = Node::new(1);
        let mut current = &mut root;
        for i in 2..=1500 {
            current.add_child(Node::new(i));
            let last_index = current.children.len() - 1;
            current = current.children[last_index].as_mut();
        }
        let result = panic::catch_unwind(|| process_tree(&root));
        match result {
            Ok(opt) => {
                assert!(opt.is_none(), "Expected operation to safely abort (return None) due to recursion depth limit, but got: {:?}", opt);
            },
            Err(_) => {
                panic!("Execution panicked due to uncontrolled recursion vulnerability.");
            }
        }
    }
}
