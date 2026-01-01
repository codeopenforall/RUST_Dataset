    Config {
        port: 8080,
        host: "127.0.0.1".to_string(),
            assert!(
                validate_config(&shared_clone),
                "Configuration validation failed in secondary thread."
            );
            println!("Configuration validated in secondary thread.");
    assert!(
        validate_config(&shared_cfg),
        "Configuration validation failed in main thread."
    );
    println!("Configuration validated in main thread.");
