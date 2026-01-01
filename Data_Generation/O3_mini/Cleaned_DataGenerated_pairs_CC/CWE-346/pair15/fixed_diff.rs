        if !self.approved.contains(origin) {
            return Err("Origin not authorized");
        }
        let mut url = "http://".to_owned();
