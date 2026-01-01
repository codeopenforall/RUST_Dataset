enum AuthError {
    UserNotFound(String),
    WrongPassword(String),
}
        match self {
            AuthError::UserNotFound(msg) => write!(f, "{}", msg),
            AuthError::WrongPassword(msg) => write!(f, "{}", msg),
        }
                    let _ptr = acc.password.as_ptr(); 
                } else {
                    return Err(AuthError::WrongPassword(format!(
                        "Account {} exists but password is incorrect", user
                    )));
        Err(AuthError::UserNotFound(format!(
            "User {} does not exist", user
        )))
            match e {
                AuthError::UserNotFound(_) => std::process::exit(1),
                AuthError::WrongPassword(_) => std::process::exit(2),
            }
