const XOR_KEY: u8 = 0xAA; 
fn xor_encrypt(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    unsafe {
        let ptr = input.as_ptr();
        for i in 0..input.len() {
            let byte = *ptr.add(i);
            output.push(byte ^ XOR_KEY);
        }
    }
    output
        let encrypted = xor_encrypt(data);
        encrypted
    println!("Encrypted transmitted data: {:?}", transmitted);
