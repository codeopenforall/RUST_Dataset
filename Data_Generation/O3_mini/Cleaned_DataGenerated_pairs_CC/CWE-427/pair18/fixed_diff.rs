const SECURE_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
fn establish_safe_env() -> String {
    SECURE_PATH.to_string()
    let safe_path = establish_safe_env();
        .env("PATH", safe_path)
