extern crate poc;

use poc::Lexer;

fn main() {
    let raw: Vec<u8> = b"// coment \n other      \n\0".to_vec();
 
    let _: Lexer = Lexer::from(raw.as_ptr() as *const _);
}
