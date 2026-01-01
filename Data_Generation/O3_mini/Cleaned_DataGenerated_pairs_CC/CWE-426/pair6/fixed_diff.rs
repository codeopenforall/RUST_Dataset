}
#[cfg(unix)]
fn absolute_path() -> String {
    String::from("/bin/true")
}
#[cfg(windows)]
fn absolute_path() -> String {
    String::from("C:\\Windows\\System32\\cmd.exe")
    let cmd = Arc::new(absolute_path());
