extern crate poc;
extern crate libc;

use poc::Lexer;

fn main() {
    let poc_input: *const libc::c_char = b"/// ll \n ll \0".as_ptr() as *const _;
    let mut lexer = Lexer::from(poc_input);

    while let Some(_) = lexer.next() {
    }
}
