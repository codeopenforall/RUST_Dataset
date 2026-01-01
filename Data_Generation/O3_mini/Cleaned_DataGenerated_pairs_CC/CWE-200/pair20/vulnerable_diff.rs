        write!(f, "Credential {{ username: {}, password: {} }}", self.username, self.password)
    cred: Credential, 
        write!(f, "SecError: {} with credential {:?}", self.details, self.cred)
                cred: data.clone(),
            cred: err.cred.clone(),
            eprintln!("Operation failed with error: {:?}", e);
            panic!("Critical failure with detailed state: {:?}", e);
