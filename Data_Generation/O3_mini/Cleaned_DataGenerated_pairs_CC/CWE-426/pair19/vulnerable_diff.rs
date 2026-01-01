        if env::var("USE_MALICIOUS").unwrap_or_default() == "1" {
            "malicious_helper".to_string()  
        } else {
            "helper_bin".to_string()         
        }
