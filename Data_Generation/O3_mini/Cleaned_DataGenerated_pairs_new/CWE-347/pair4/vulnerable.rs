use std::thread;
use std::ptr;
struct Record {
    content: String,
    signature: [u8; 32],
}
impl Record {
    unsafe fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
        let mut outcome = [0u8; 32];
        let bytes = self.content.as_bytes();
        for i in 0..32 {
            let c = *bytes.get_unchecked(i % bytes.len());
            outcome[i] = c ^ key[i];
        }
        outcome
    }
    fn verify(&self, key: &[u8; 32]) -> bool {
        let expected = unsafe { self.compute_expectation(key) };
        unsafe {
            let exp_ptr = expected.as_ptr();
            let sig_ptr = self.signature.as_ptr();
            for i in 0..32 {
                if *exp_ptr.add(i) != *sig_ptr.add(i) {
                    return false;
                }
            }
        }
        true
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
