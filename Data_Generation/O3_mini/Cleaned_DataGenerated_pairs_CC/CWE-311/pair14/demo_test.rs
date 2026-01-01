// Test oracle for verifying secure transmission behavior.
// This function should be used in a controlled test environment. It starts a TCP listener on port 8080,
// writes a known secret to "config.txt", invokes the main function of the application,
// and then asserts that the data received is the encrypted version of the secret.
// For the insecure implementation, the test will fail because the data is in clear text.
// For the secure implementation, the test will pass.

#[cfg(test)]
mod tests {
    use std::fs::write;
    use std::io::Read;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;

    // Helper: start a TCP listener on 127.0.0.1:8080.
    fn spawn_listener() -> TcpListener {
        TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080")
    }

    // Oracle test that triggers the application and checks the transmitted data.
    #[test]
    fn oracle() {
        // Write the known secret into config.txt.
        // The secret is known (e.g., "s3cret") to derive the expected encrypted value.
        let secret = "s3cret";
        write("config.txt", secret).expect("Failed to write config.txt");
        
        let listener = spawn_listener();
        
        // Spawn the application in a separate thread.
        // In practice, ensure that the appropriate binary (insecure or secure) is built.
        thread::spawn(|| {
            // The main function from the application is called.
            // Depending on which version is tested, this will be the insecure or secure variant.
            crate::main();
        });
        
        // Allow some time for the connection to be established.
        thread::sleep(Duration::from_millis(100));
        
        // Accept the incoming connection.
        let (mut stream, _) = listener.accept().expect("Did not receive any connection");
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).expect("Failed to read from stream");
        
        // For the secure version, an XOR with 0xAA is applied.
        let expected: Vec<u8> = secret.bytes().map(|b| b ^ 0xAA).collect();
        
        // The test asserts that the transmitted data matches the expected encrypted data.
        // For the insecure version, the assertion will fail.
        assert_eq!(buffer, expected, "Transmitted data does not match the encrypted expected value");
    }
}