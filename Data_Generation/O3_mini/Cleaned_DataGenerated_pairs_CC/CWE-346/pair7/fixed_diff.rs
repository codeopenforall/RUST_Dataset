static TRUSTED_ORIGIN: &str = "https://trusteddomain.com";
fn validate(req: &HttpRequest) -> bool {
    req.origin == TRUSTED_ORIGIN
        validate(&req)
