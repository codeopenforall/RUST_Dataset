#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_endpoint_auth() {
        // Even if the configuration debug flag is set, the endpoint should require proper authentication.
        let config = Arc::new(Config {
            debug: true, // In the fixed version, this flag no longer allows bypass.
            token: "secret123".to_string(),
        });
        let server = Server::new(config);
        // For the debug endpoint, providing no token must result in authentication failure.
        let response = server.process_req("/debug", None);
        assert_eq!(response, false, "The debug endpoint should not bypass authentication");
    }
}