extern crate poc;
extern crate libc;

use poc::Lexer;

fn main() {
    let mut lexer = Lexer::from(b"lll \n ll".to_vec());

    while let Some(_) = lexer.next() {
        unsafe {
            println!("{:?}", poc::ffi::yypos);
        }
    }
}
