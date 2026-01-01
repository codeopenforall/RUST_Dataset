    fn new(addr: &str, port: u16, max_conn: usize) -> Self {
        Settings {
            addr: addr.to_owned(),
            port,
            max_conn,
        }
    Settings::new("127.0.0.1", 443, 100).calc()
