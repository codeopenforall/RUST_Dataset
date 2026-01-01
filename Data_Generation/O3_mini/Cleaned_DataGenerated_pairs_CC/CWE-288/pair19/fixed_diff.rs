        if endpoint == "/secure" || endpoint == "/debug" {
        debug: false,
        let resp = server.process_req("/debug", Some("secret123"));
