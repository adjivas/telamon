extern crate poc;
extern crate libc;

//use poc::Lexer;
use std::{mem,ptr};

fn main() {
    unsafe {
        let poc_input: *const libc::c_char = b"/// ll \n ll \0".as_ptr() as *const _;
        let poc_scanner: poc::yyscan_t = ptr::null();

        poc::yylex_init(&poc_scanner);

        let poc_buffer_state: poc::yy_buffer_state = poc::yy_scan_string(poc_input, poc_scanner);
        loop {
            match poc::yylex(poc_scanner) {
		libc::EOF => break ,
                code => {
			println!("lines: {}, code: {}", poc::lines, code);
			let out: *const libc::c_char = poc::yyget_text(poc_scanner);
			libc::write(1, out as *const _, libc::strlen(out as *const _));
                },
            }
        }
	println!();
        poc::yy_delete_buffer(poc_buffer_state, poc_scanner);
        poc::yylex_destroy(poc_scanner);
    }
}
