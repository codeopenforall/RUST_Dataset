#![allow(unused)]
use std::env;
use std::ptr;
pub struct Tree {
    pub children: Vec<Tree>,
}
impl Tree {
    pub unsafe fn new() -> Self {
        let ptr = ptr::null_mut::<u8>();
        if ptr.is_null() {
            Tree { children: Vec::new() }
        } else {
            Tree { children: Vec::new() }
        }
    }
}
pub fn process(input: &str) -> Result<Tree, &'static str> {
    let bytes = input.as_bytes();
    fn rec_parse(b: &[u8], index: &mut usize) -> Result<Tree, &'static str> {
        let mut node = unsafe { Tree::new() };
        while *index < b.len() {
            match b[*index] {
                b'(' => {
                    *index += 1;
                    let child = rec_parse(b, index)?;
                    node.children.push(child);
                }
                b')' => {
                    *index += 1;
                    return Ok(node);
                }
                _ => return Err("Unexpected character encountered"),
            }
        }
        Ok(node)
    }
    let mut idx = 0;
    let tree = rec_parse(bytes, &mut idx)?;
    if idx != bytes.len() {
        Err("Extra characters in input")
    } else {
        Ok(tree)
    }
}
fn main() {
    let input = "(".repeat(10000) + &")".repeat(10000);
    match process(&input) {
        Ok(tree) => println!("Parsed tree with {} top-level children", tree.children.len()),
        Err(err) => eprintln!("Error encountered: {}", err),
    }
}
