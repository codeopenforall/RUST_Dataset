            if rec.username == name {
                if rec.pass_hash == input_hash {
                    return Ok("Access granted");
                } else {
                    return Err("Password mismatch");
                }
        Err("User not found")
