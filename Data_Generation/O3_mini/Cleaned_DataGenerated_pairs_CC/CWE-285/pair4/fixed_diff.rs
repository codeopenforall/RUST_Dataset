fn obtain_data(cred: &str) -> Result<i32, &'static str> {
    if cred != "admin" {
        return Err("Unauthorized");
    }
