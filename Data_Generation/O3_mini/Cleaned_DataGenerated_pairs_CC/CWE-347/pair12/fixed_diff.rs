        if domain != "expected_domain" {
            return false;
        if signature.len() != 14 {
            return false;
        }
        true
    let signature = b"valid_signatur"; 
    let domain = "expected_domain";
        assert!(valid, "Verification should succeed");
