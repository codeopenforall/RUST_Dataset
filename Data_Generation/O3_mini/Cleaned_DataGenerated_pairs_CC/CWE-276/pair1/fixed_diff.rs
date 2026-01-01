        let mut file = OpenOptions::new()
                    .mode(0o600)
        let guard = self.data.lock().unwrap();
        let safe_slice = &guard[5..10];
        file.write_all(safe_slice)?;
                        .mode(0o600)
