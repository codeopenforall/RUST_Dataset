/*
The following test oracle function verifies that the transmitted data does not equal
the cleartext secret. When executed against the insecure implementation, the test will fail
(as the plain secret "myS3cr3t!" is transmitted), while it will pass against the secure version.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmission_must_not_be_plaintext() {
        // The expected sensitive secret.
        let secret = "myS3cr3t!";
        // Invoke the transmission function from the implementation.
        let transmitted = get_transmitted_data();
        // The transmitted data must not match the cleartext secret.
        assert_ne!(
            transmitted, secret,
            "Failure: Transmitted data is in cleartext!"
        );
    }
}