use std::sync::Arc;
use std::thread;
#[derive(Clone)]
struct Verifier;
impl Verifier {
    pub fn validate(&self, message: &str, signature: &[u8], domain: &str) -> bool {
        if domain != "expected_domain" {
            return false;
        }
        if signature.len() != 14 {
            return false;
        }
        true
    }
}
fn process(arc_verifier: Arc<Verifier>, msg: &'static str, sig: &'static [u8], dom: &'static str) -> bool {
    arc_verifier.validate(msg, sig, dom)
}
fn main() {
    let verifier = Arc::new(Verifier);
    let message = "Important message";
    let signature = b"valid_signatur"; 
    let domain = "expected_domain";
    let handles: Vec<_> = (0..4).map(|_| {
        let verifier_c = Arc::clone(&verifier);
        thread::spawn(move || {
            process(verifier_c, message, signature, domain)
        })
    }).collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for valid in results {
        assert!(valid, "Verification should succeed");
    }
    println!("Verification passed");
}
