#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_boundary() {
        let proc_inst = Processor { data: std::sync::Arc::new(vec![10, 20, 30, 40]) };
        let result = panic::catch_unwind(|| {
            proc_inst.compute(3, 2)
        });
        match result {
            Ok(inner) => {
                if let Some(res) = (&inner as &dyn std::any::Any).downcast_ref::<Result<i32, &str>>() {
                    assert_eq!(*res, Err("Index out-of-bounds"), "Expected an error for out-of-bound access");
                } else {
                    panic!("Test failed: The function returned a non-Result value, indicating a vulnerability.");
                }
            }
            Err(_) => {
                panic!("Test failed: The function panicked due to out-of-bounds access, indicating a vulnerability.");
            }
        }
    }
}
