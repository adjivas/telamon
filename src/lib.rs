extern crate libc;

use std::mem;

/// https://westes.github.io/flex/manual/About-yyscan_005ft.html
type yyscan_t = libc::c_void;

type yy_buffer_state = *mut libc::c_void;
type YY_BUFFER_STATE = yy_buffer_state;

extern {
    pub static lines: libc::c_int;

    fn yylex_init(scanner: *mut yyscan_t) -> libc::c_int;
    fn yy_scan_string(yy_str: *const libc::c_char, yyscanner: *mut yyscan_t) -> YY_BUFFER_STATE;
    fn yylex(yyscanner: *mut yyscan_t) -> libc::c_int;
    fn yy_delete_buffer(b: YY_BUFFER_STATE, yyscanner: *mut yyscan_t);
    fn yylex_destroy(yyscanner: *mut yyscan_t) -> libc::c_int;
}

pub struct Lexer {
    scanner: *mut yyscan_t,
    buffer: YY_BUFFER_STATE,
}

impl From<*const libc::c_char> for Lexer {
    /// https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
    fn from(input: *const libc::c_char) -> Self {
        unsafe {
            let mut scanner: yyscan_t = mem::uninitialized();

            println!("{}",yylex_init(&mut scanner) );
            Lexer {
                scanner: &mut scanner,
                buffer: mem::uninitialized(),
                //yy_scan_string(input, &mut scanner),
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = libc::c_int;

    fn next(&mut self) -> Option<Self::Item> {

        unsafe {
        println!("ss");
        let code = yylex(self.scanner);
        println!("{:?}", code);
            match code {
                libc::EOF => None,
                code => Some(code),
            }
        }
    }
}

impl Drop for Lexer {
    fn drop(&mut self) {
        unsafe {
//          println!("{:?}",  yy_delete_buffer(self.buffer, self.scanner) );
//         println!("{:?}",   yylex_destroy(self.scanner) );
        }
    }
}
