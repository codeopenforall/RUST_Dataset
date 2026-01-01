#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let cfg = make_config();
        assert_eq!(cfg.label, "default", "Configuration 'label' should be 'default'");
    }
}
