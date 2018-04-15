extern crate libc;

mod ffi;

use std::ptr;

pub struct Lexer {
    scanner: ffi::YyScan,
    buffer: ffi::YyBufferState,
}

impl From<*const libc::c_char> for Lexer {
    fn from(input: *const libc::c_char) -> Self {
        unsafe {
            let scanner: ffi::YyScan = ptr::null();

            ffi::yylex_init(&scanner); // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
            Lexer {
                scanner: scanner,
                buffer: ffi::yy_scan_string(input, scanner), // https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
            }
        }
    }
}

impl Iterator for Lexer {
    type Item = libc::c_int;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match ffi::yylex(self.scanner) {
                libc::EOF => None,
                code => {
                    let out: *const libc::c_char = ffi::yyget_text(self.scanner);

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
	        libc::write(1, b"\n".as_ptr() as *const _, 1);
            ffi::yy_delete_buffer(self.buffer, self.scanner); // https://westes.github.io/flex/manual/Multiple-Input-Buffers.html
            ffi::yylex_destroy(self.scanner); // https://westes.github.io/flex/manual/Init-and-Destroy-Functions.html#index-yylex_005finit
        }
    }
}
