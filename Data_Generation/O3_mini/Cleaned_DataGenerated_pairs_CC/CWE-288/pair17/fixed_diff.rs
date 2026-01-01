         if let Some((_, token)) = req.headers.iter().find(|(k, _)| k == "X-Debug") {
             if let Ok(valid_token) = env::var("DEBUG_TOKEN") {
                 if token == &valid_token {
                     return self.check_user(&req.username, &req.password);
             return false;
             headers: vec![("X-Debug".to_string(), "wrong".to_string())],
             println!("Debug route: Authentication succeeded (unexpected)");
             println!("Debug route: Authentication correctly rejected");
         password: "password".to_string(),
