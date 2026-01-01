            if origin.contains('\0') {
                return false;
            origin == trusted
    hdrs.insert("Origin".to_string(), "https://example.com\0.attacker".to_string());
