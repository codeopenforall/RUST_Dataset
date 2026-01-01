#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snapshot_correctness() {
        let inst = Container::new();
        // Append a known value.
        inst.process(100).expect("Write failed");
        let snap = inst.inspect();
        // Expect exactly one element with the value 100.
        assert_eq!(snap, vec![100]);
    }
}