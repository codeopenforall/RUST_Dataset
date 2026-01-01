fn xor_cipher(input: &str, key: u8) -> Vec<u8> {
    input.bytes().map(|b| b ^ key).collect()
}
fn transmit(data: &[u8]) -> io::Result<()> {
    stream.write_all(data)?;
    let encrypted = xor_cipher(&data, 0xAA);
         if let Err(e) = transmit(&encrypted) {
