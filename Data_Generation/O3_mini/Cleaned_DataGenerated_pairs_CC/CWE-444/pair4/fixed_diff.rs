    headers: Vec<String>,
    fn new(raw: &str) -> Self {
        let headers = raw.split("\r\n").map(|s| s.to_string()).collect();
        Message { headers }
    fn extract_length(&self) -> u32 {
        let mut reported: Option<u32> = None;
        for line in &self.headers {
            if let Some(rest) = line.strip_prefix("Content-Length:") {
                let token = rest.trim();
                if let Ok(num) = token.parse::<u32>() {
                    if let Some(existing) = reported {
                        if existing != num {
                            return 0;
                    } else {
                        reported = Some(num);
        reported.unwrap_or(0)
    let msg = Message::new(data);
    msg.extract_length()
    let valid = "POST / HTTP/1.1\r\nHost: secure\r\nContent-Length: 30\r\n\r\n";
    let value = simulate(valid);
