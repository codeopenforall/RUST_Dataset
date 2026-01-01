fn extract_host(origin: &str) -> Option<&str> {
    let parts: Vec<&str> = origin.split("://").collect();
    if parts.len() != 2 {
        return None;
    }
    let remainder = parts[1];
    let host_port = if let Some(at_pos) = remainder.find('@') {
        &remainder[at_pos + 1..]
    } else {
        remainder
    };
    let host = host_port.split('/').next().unwrap_or("");
    Some(host)
}
        match extract_host(origin) {
            Some(host) => host == self.config,
            None => false,
        }
