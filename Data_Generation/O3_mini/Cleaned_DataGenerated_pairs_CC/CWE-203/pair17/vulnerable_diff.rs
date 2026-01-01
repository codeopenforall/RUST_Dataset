    UserMissing,
    CredentialMismatch,
        if let Some(valid_secret) = users.get(account) {
                if valid_str == secret {
                    return Ok(());
                } else {
                    return Err(ErrorType::CredentialMismatch);
                }
            return Err(ErrorType::UserMissing);
