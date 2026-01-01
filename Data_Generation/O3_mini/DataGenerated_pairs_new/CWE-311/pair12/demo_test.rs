/*
This test oracle validates the secure transmission behavior.

For the vulnerable version:
 - The process function will return an error since the sensitive string "super_secret_password"
   remains in cleartext and triggers the error condition in the send function.
 
For the fixed version:
 - The process function encrypts the sensitive data, ensuring it does not contain the substring "secret".
   Hence, the send function should complete successfully and return Ok.

The test below asserts that the result from run() (acting as process) is Ok.
*/

#[cfg(test)]
mod tests {
    // Import all items from the parent module.
    use super::*;

    #[test]
    fn test_secure_transmission() {
        // run() should yield Ok() in the secure (fixed) version,
        // and fail (i.e. yield Err) in the vulnerable version.
        let result = run();
        assert!(result.is_ok(), "Test failed: Transmission did not secure the sensitive data.");
    }
}