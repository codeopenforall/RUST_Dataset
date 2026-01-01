        write!(f, "Config {{ secret: [REDACTED] }}")
                // FIX: avoid disclosing the actual secret by using the corrected debug output.
                eprintln!("Error: configuration encountered an error, config: {:?}", config_clone);
