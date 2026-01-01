            if rec.username == name && rec.pass_hash == input_hash {
                return Ok("Access granted");
        Err("Invalid credentials")
