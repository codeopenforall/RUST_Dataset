        token == "secret_token"
        assert!(!srv_clone.validate("debug"), "Security check failed: debug token must not bypass authentication!");
    assert!(server.validate("secret_token"), "Valid token was rejected unexpectedly!");
    println!("Running fixed secure server application.");
