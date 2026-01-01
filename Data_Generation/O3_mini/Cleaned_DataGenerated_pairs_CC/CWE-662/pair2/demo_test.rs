#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn oracle_test() {
        let ctrl = Controller::new();
        let shared_ctrl = Arc::new(ctrl);
        let thread_count = 4;
        let iterations = 10_000;
        let expected = thread_count * iterations;

        let mut handles = vec![];
        for _ in 0..thread_count {
            let ctl = Arc::clone(&shared_ctrl);
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    ctl.update();
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let result = shared_ctrl.get_count();
        assert_eq!(result, expected, "Final count does not match expected value");
    }
}