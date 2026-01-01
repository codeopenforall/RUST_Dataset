    fn rec_parse(b: &[u8], index: &mut usize) -> Result<Tree, &'static str> {
                    let child = rec_parse(b, index)?;
    let tree = rec_parse(bytes, &mut idx)?;
    let input = "(".repeat(10000) + &")".repeat(10000);
