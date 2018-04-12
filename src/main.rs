extern crate poc;

use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
        let raw = b"// coment \n other      \n \0".to_vec();
        
        unsafe {
            let input = CString::from_vec_unchecked(raw);
            let mut lex = poc::Lexer::from(input.as_ptr());

            println!("{:?}", lex.next() );
        }
}
