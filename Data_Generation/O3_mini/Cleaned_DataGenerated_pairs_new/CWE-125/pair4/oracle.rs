#[cfg(test)]
mod tests {
    use std::panic;
    use crate::app::Processor;
    #[test]
    fn test_out_of_bounds() {
        let proc = Processor::new(vec![1, 2, 3]);
        let result = panic::catch_unwind(|| {
            proc.compute(3)
        });
        match result {
            Ok(opt) => {
                assert_eq!(opt, None, "Expected None for out-of-bounds access");
            }
            Err(_) => {
                panic!("Out-of-bounds access triggered a panic");
            }
        }
    }
}
