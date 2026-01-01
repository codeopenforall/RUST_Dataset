#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snapshot_correctness() {
        let inst = Container::new();
        inst.process(100).expect("Write failed");
        let snap = inst.inspect();
        assert_eq!(snap, vec![100]);
    }
}
