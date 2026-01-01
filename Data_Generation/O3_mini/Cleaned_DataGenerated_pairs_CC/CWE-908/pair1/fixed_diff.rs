    fn load() -> Self {
        Settings {
            timeout: 30,
            retries: 3,
        }
    let shared = Arc::new(Settings::load());
            println!("Retries is zero, value: {}", cloned.retries);
