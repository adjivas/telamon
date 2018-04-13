extern crate libc;

use std::mem;
use std::ptr;

/// https://westes.github.io/flex/manual/About-yyscan_005ft.html
pub type yyscan_t = *const libc::c_void;
pub type yy_buffer_state = *const libc::c_void;

extern {
    pub static lines: libc::c_int;

    pub fn yylex_init(scanner: *const yyscan_t) -> libc::c_int;
    pub fn yy_scan_string(yy_str: *const libc::c_char, yyscanner: yyscan_t) -> yy_buffer_state;
    pub fn yylex(yyscanner: yyscan_t) -> libc::c_int;
    pub fn yyget_text(yyscanner: yyscan_t) -> *const libc::c_char;
    pub fn yy_delete_buffer(b: yy_buffer_state, yyscanner: yyscan_t);
    pub fn yylex_destroy(yyscanner: yyscan_t) -> libc::c_int;

    pub static poc_input: *const libc::c_char;
    pub static poc_scanner: yyscan_t;
    pub static poc_buffer_state: yy_buffer_state;
}

pub struct Lexer {
    scanner: yyscan_t,
    buffer: yy_buffer_state,
}

impl From<*const libc::c_char> for Lexer {
    /// https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
    fn from(input: *const libc::c_char) -> Self {
        unsafe {
            let scanner: yyscan_t = ptr::null();

            yylex_init(&scanner);
            Lexer {
                scanner: scanner,
                buffer: yy_scan_string(input, scanner),
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = libc::c_int;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match yylex(poc_scanner) {
		libc::EOF => None,
                code => {
			println!("lines: {}, code: {}", lines, code);
			let out: *const libc::c_char = yyget_text(self.scanner);
			libc::write(1, out as *const _, libc::strlen(out as *const _));
			Some(code)
                },
           }
        }
    }
}

impl Drop for Lexer {
    fn drop(&mut self) {
	unsafe {
	        yy_delete_buffer(self.buffer, self.scanner);
        	yylex_destroy(self.scanner);
	}
    }
}
