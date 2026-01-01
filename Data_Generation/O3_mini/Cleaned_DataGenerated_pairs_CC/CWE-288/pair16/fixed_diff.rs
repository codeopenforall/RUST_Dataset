struct SecurityHandler {
    debug: bool,
}
        SecurityHandler { debug: false }
        if self.debug {
            if let Some(flag) = req.headers.get("X-Debug-Bypass") {
                if flag == "true" {
                    println!("Debug header detected, but bypass is disabled.");
    let mut headers = std::collections::HashMap::new();
