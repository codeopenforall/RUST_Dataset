#[cfg(test)]
mod tests {
    use super::Controller;
    #[test]
    fn invariant_test() {
        let mut ctrl = Controller::new();
        for _ in 0..10 {
            ctrl.update();
        }
        assert!(ctrl.counter >= 0, "Invariant violated: counter is {}", ctrl.counter);
    }
}
