use std::thread;
const DOMAIN: &[u8] = b"DOC"; 
struct Record {
    content: String,
    signature: [u8; 32],
}
impl Record {
    fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut outcome = [0u8; 32];
        let bytes = self.content.as_bytes();
        for i in 0..32 {
            let domain_byte = DOMAIN[i % DOMAIN.len()];
            let content_byte = bytes[i % bytes.len()];
            outcome[i] = content_byte ^ key[i] ^ domain_byte;
        }
        outcome
    }
    fn verify(&self, key: &[u8; 32]) -> bool {
        let expected = self.compute_expectation(key);
        expected == self.signature
    }
}
fn main() {
    let public_key: [u8; 32] = [0xAB; 32];
    let content = String::from("Attack");
    let mut forged_signature = [0u8; 32];
    {
        let bytes = content.as_bytes();
        for i in 0..32 {
            forged_signature[i] = bytes[i % bytes.len()] ^ public_key[i];
        }
    }
    let record = Record {
        content,
        signature: forged_signature,
    };
    let handle = thread::spawn(move || {
        if record.verify(&public_key) {
            println!("Verification succeeded");
        } else {
            println!("Verification failed");
        }
    });
    handle.join().unwrap();
}
