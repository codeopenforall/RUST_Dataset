//////////////// Test Oracle Begin //////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The test oracle creates a scenario in which the spawned task takes longer than the allowed timeout.
    // Thus, a correct implementation should return an error, while the vulnerable version erroneously returns Ok.
    #[test]
    fn oracle() {
        let proc = Procedure { compute_delay: 200, timeout: 100 };
        let outcome = proc.process();
        assert!(
            outcome.is_err(),
            "Oracle: Expected error on timeout, but got a successful result: {:?}",
            outcome
        );
    }
}
//////////////// Test Oracle End //////////////////