////////////////////////////////////////////////////////////////////////////////
// The following test oracle function is intended to be compiled separately 
// (and linked appropriately) when testing either the vulnerable or the fixed 
// versions. It calls the provided helper function "check_command" with a relative 
// path. In the fixed version, this must return an error "Command must be absolute path".
// In the vulnerable version, it will attempt to launch the relative command and 
// return a different error message. This difference in behavior causes the test 
// to fail for the vulnerable version and pass for the fixed version.
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    // Import the helper function "check_command" from the respective crate.
    use super::check_command;

    #[test]
    fn untrusted_search_path_oracle() {
        let result = check_command("relative_cmd");
        match result {
            Err(ref msg) if msg == "Command must be absolute path" => {
                // Expected failure: the fixed version correctly rejects relative paths.
            },
            _ => {
                panic!("Test failed: security check did not prevent untrusted search path.");
            }
        }
    }
}