extern crate libc;

use std::mem;

/// https://westes.github.io/flex/manual/About-yyscan_005ft.html
type yyscan_t = *mut libc::c_void;
type yy_buffer_state = *mut libc::c_void;

extern {
    static lines: libc::c_int;

    fn poc_yylex_init(scanner: yyscan_t) -> libc::c_int;
    fn yylex_init(scanner: yyscan_t) -> libc::c_int;
    fn yy_scan_string(yy_str: *const libc::c_char, yyscanner: yyscan_t) -> yy_buffer_state;
    fn yylex(yyscanner: yyscan_t) -> libc::c_int;
    fn yy_delete_buffer(b: yy_buffer_state, yyscanner: yyscan_t);
    fn yylex_destroy(yyscanner: yyscan_t) -> libc::c_int;

    fn ff(input: *const libc::c_char /*, scanner: yyscan_t */);
}

pub struct Lexer {
//    scanner: yyscan_t,
//    buffer: yy_buffer_state,
}

impl From<*const libc::c_char> for Lexer {
    /// https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
    fn from(input: *const libc::c_char) -> Self {
        unsafe {
            let scanner: yyscan_t = mem::zeroed();

//            poc_yylex_init(scanner);
            ff(input);
//	    let buffer = mem::zeroed();
//            let buffer = yy_scan_string(input, scanner);
            Lexer {
//                scanner: scanner,
//                buffer: buffer,
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = libc::c_int;

    fn next(&mut self) -> Option<Self::Item> {
	None
        /*unsafe {
            let code = yylex(self.scanner);
            println!("{:?}", code);
            match code {
                libc::EOF => None,
                code => Some(code),
            }
        }*/
    }
}

impl Drop for Lexer {
    fn drop(&mut self) {
        /*unsafe {
            yy_delete_buffer(self.buffer, self.scanner);
            yylex_destroy(self.scanner);
        }*/
    }
}
