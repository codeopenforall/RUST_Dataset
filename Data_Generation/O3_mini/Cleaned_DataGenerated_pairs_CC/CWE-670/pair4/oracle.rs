#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        let service = Service::new();
        match service.process("danger") {
            Ok(val) => panic!("Test failed: expected error, got Ok({})", val),
            Err(e) => assert_eq!(e, "Danger command not allowed"),
        }
    }
}
