extern crate libc;

// e yyscan_t = libc::c_int;
// e YY_BUFFER_STATE = libc::c_int;

extern {
    pub static lines: libc::c_int;

//  fn yylex_init(yyscan_t *mut scanner) -> libc::c_int;
//    fn yy_delete_buffer(YY_BUFFER_STATE, yyscan_t *mut scanner) -> libc::c_int;
}

/*pub struct Lexer {
    buffer: YY_BUFFER_STATE,
    yyscan_t scanner,
}

impl Default for Lexer {
    fn default() -> Self {
        unsafe {
            yyscan_t scanner;
            yylex_init(&scanner);
        }

        Lexer {
            buffer: buffer,
            scanner: scanner,
        }
    }
}

impl Drop for Lexer {
    fn drop(&mut self) {
        yy_delete_buffer(self.buffer, self.scanner);
    }
}*/
