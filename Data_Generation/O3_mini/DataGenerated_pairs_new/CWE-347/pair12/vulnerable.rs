//////////////////////////////////////////////////////////////
// Vulnerable Code for Improper Cryptographic Signature Verification
//////////////////////////////////////////////////////////////
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct Verifier;

impl Verifier {
    // This method is intended to verify a digital signature for a message
    // with a given domain context. However, due to an unsafe misuse,
    // it neglects to properly verify the supplied domain.
    pub fn validate(&self, message: &str, signature: &[u8], domain: &str) -> bool {
        unsafe {
            // Simulate usage of low-level operations on the domain string
            // to mimic real-world unsafe interactions.
            let ptr = domain.as_ptr();
            // Deliberately creating a slice from raw bytes (without validating content)
            let bytes = std::slice::from_raw_parts(ptr, domain.len());
            let _ = std::str::from_utf8_unchecked(bytes);
            // Vulnerability: The proper domain check is skipped.
            // Even if the domain does not match the expected value,
            // the signature gets accepted.
            return true;
        }
    }
}

fn process(arc_verifier: Arc<Verifier>, msg: &'static str, sig: &'static [u8], dom: &'static str) -> bool {
    arc_verifier.validate(msg, sig, dom)
}

fn main() {
    // In this vulnerable implementation, the domain is set to an incorrect value.
    // The unsafe verify routine erroneously accepts any domain.
    let verifier = Arc::new(Verifier);
    let message = "Important message";
    let signature = b"fake_signature"; // Dummy signature bytes.
    let domain = "invalid_domain"; // Should have been "expected_domain"

    let handles: Vec<_> = (0..4).map(|_| {
        let verifier_c = Arc::clone(&verifier);
        thread::spawn(move || {
            process(verifier_c, message, signature, domain)
        })
    }).collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    // If any thread returns false, exit with failure.
    for valid in results {
        if !valid {
            println!("Verification error");
            std::process::exit(1);
        }
    }
    println!("Verification passed");
}