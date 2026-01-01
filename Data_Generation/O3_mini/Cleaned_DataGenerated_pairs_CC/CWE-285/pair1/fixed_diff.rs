        match self.role {
            UserRole::Admin => Ok(unsafe { self.unsafe_get_secret() }),
            _ => Err("Access denied"),
        }
