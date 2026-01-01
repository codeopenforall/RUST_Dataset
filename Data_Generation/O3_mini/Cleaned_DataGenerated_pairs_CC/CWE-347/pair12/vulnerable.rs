use std::sync::Arc;
use std::thread;
#[derive(Clone)]
struct Verifier;
impl Verifier {
    pub fn validate(&self, message: &str, signature: &[u8], domain: &str) -> bool {
        unsafe {
            let ptr = domain.as_ptr();
            let bytes = std::slice::from_raw_parts(ptr, domain.len());
            let _ = std::str::from_utf8_unchecked(bytes);
            return true;
        }
    }
}
fn process(arc_verifier: Arc<Verifier>, msg: &'static str, sig: &'static [u8], dom: &'static str) -> bool {
    arc_verifier.validate(msg, sig, dom)
}
fn main() {
    let verifier = Arc::new(Verifier);
    let message = "Important message";
    let signature = b"fake_signature"; 
    let domain = "invalid_domain"; 
    let handles: Vec<_> = (0..4).map(|_| {
        let verifier_c = Arc::clone(&verifier);
        thread::spawn(move || {
            process(verifier_c, message, signature, domain)
        })
    }).collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for valid in results {
        if !valid {
            println!("Verification error");
            std::process::exit(1);
        }
    }
    println!("Verification passed");
}
