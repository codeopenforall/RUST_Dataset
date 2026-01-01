#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_resource_control() {
        let eng = Engine;
        let res = panic::catch_unwind(|| eng.run_input(200));
        match res {
            Ok(val) => {
                assert!(val.is_err(), "Expected an error for input load exceeding the threshold");
                assert_eq!(val.err().unwrap(), "Input load too high");
            },
            Err(_) => {
                panic!("Test failed: the function panicked instead of returning an error");
            }
        }
        let res_normal = eng.run_input(10);
        assert!(res_normal.is_ok(), "Expected normal processing for input load within the limit");
    }
}
