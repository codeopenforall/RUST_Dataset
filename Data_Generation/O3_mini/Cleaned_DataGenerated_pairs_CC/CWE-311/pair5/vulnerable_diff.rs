    let secret_ptr: *const String = &cfg.secret;
        let data = (*secret_ptr).clone();
        SENT_DATA = Some(data);
