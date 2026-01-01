#[cfg(test)]
mod tests {
    use super::Controller;
    #[test]
    fn check_control_flow() {
        let ctrl = Controller::new(1);
        let res = ctrl.update(10);
        match res {
            Ok(val) => assert_eq!(val, 11, "Expected valid result of 11"),
            Err(err) => panic!("Test failed due to control flow error: {}", err),
        }
    }
}
