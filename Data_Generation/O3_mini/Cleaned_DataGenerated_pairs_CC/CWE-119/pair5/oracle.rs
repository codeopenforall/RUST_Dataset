#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn oracle_test() {
        let integrity = run();
        assert!(integrity, "Integrity check failed: sentinel was corrupted");
    }
}
