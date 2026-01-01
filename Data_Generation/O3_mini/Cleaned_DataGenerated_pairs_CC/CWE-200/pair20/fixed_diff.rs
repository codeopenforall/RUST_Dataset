        write!(f, "Credential {{ username: {}, password: [REDACTED] }}", self.username)
        write!(f, "SecError: {}", self.details)
            eprintln!("Operation failed with error: {}", e);
            panic!("Critical failure encountered");
