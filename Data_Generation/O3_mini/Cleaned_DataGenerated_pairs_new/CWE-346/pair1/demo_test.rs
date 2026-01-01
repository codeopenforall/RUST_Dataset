#[cfg(test)]
mod tests {
    // Bring in the definitions from the module under test.
    use super::*;

    #[test]
    fn test_origin_check() {
        let service = Service {
            allowed_origin: "https://trusted.com".into(),
        };
        let forged_req = Request {
            origin: "https://untrusted.com".into(),
            payload: "dummy".into(),
        };
        let result = service.handle(forged_req);
        // The contract: a forged origin must result in rejection (i.e., false).
        assert!(!result, "Origin check failed: untrusted origin was approved");
    }
}