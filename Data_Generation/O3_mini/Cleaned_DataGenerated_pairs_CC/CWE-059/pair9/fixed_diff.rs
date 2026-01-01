const ALLOWED_PREFIX: &str = "./safe/";
    if !resolved.starts_with(ALLOWED_PREFIX) {
        return Err(String::from("Access denied due to illegal path"));
    }
