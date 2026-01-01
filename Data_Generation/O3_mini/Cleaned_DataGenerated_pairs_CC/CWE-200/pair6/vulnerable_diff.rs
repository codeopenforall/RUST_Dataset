        write!(f, "Config {{ secret: {} }}", self.secret)
                // FLAW: printing the debug representation inadvertently discloses sensitive data.
                eprintln!("Error: encountered configuration error in {:?}", config_clone);
