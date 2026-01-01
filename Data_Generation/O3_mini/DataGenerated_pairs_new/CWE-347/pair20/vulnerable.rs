//////////////// Vulnerable Code ////////////////////
use std::sync::Arc;
use std::thread;

struct Packet {
    data: Vec<u8>,
    sig: Vec<u8>,
}

impl Packet {
    // This function attempts to verify a cryptographic signature but does so
    // using unsafe code that unsafely casts the signature slice to a fixed-size array.
    // It does not validate that the signature buffer is of the proper length,
    // nor does it check the algorithm or perform any real verification.
    fn check(&self) -> bool {
        unsafe {
            // Vulnerability: Unsafely assuming the signature is 64 bytes.
            // If the signature buffer is less than 64 bytes, this leads to undefined behavior.
            let ptr = self.sig.as_ptr() as *const [u8; 64];
            let _sig_arr = *ptr; // This transmutation is improperly verified.
            // Improper verification: Always returns true for non-empty data,
            // ignoring errors in the signature content.
            self.data.len() > 0
        }
    }
}

fn main() {
    // In this example, the signature buffer has an incorrect size.
    // Despite being invalid, the flawed verification erroneously passes.
    let pkg = Arc::new(Packet {
         data: b"Example message".to_vec(),
         sig: vec![0u8; 32], // Incorrect signature length intentionally.
    });
    let pkg2 = Arc::clone(&pkg);
    let handle = thread::spawn(move || {
         if !pkg2.check() {
             panic!("Invalid cryptographic check!");
         }
    });
    handle.join().unwrap();
    println!("Processing complete");
}