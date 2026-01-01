        match user {
            "admin" => {
                self.access = true;
            },
            "guest" => {
                self.access = false;
            },
            _ => {
                self.access = false;
            },
