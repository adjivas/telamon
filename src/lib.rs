extern crate libc;

pub mod ffi;

use std::ptr;

pub struct Lexer {
    scanner: ffi::YyScan,
    buffer: ffi::YyBufferState,
}

impl From<Vec<u8>> for Lexer {
    fn from(buffer:Vec<u8>) -> Self {
        unsafe {
            let scanner: ffi::YyScan = ptr::null();

            ffi::yylex_init(&scanner); // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
            Lexer {
                scanner: scanner,
                buffer: ffi::yy_scan_bytes(buffer.as_ptr() as *const _, buffer.len() as _, scanner), // https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = ffi::YyToken;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match ffi::yylex(self.scanner) {
                ffi::YyToken::EOF => None,
                code => Some(code),
            }
        }
    }
}

impl Drop for Lexer {
    fn drop(&mut self) {
        unsafe {
            ffi::yy_delete_buffer(self.buffer, self.scanner); // https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
            ffi::yylex_destroy(self.scanner); // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
        }
    }
}
