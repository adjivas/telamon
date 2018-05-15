#![allow(dead_code)]

use ::libc;

/// https://westes.github.io/flex/manual/About-yyscan_005ft.html
pub type YyScan = *const libc::c_void;
pub type YyBufferState = *const libc::c_void;
pub type YySize = libc::size_t;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum YyToken {
    Var,
    EOF = libc::EOF as _,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct YyPos {
    line: libc::c_uint,
    column: libc::c_uint,
}

extern {
    pub static yypos: YyPos;

    pub fn yylex_init(scanner: *const YyScan) -> libc::c_int;
    pub fn yy_scan_string(yy_str: *const libc::c_char, yyscanner: YyScan) -> YyBufferState;
    pub fn yy_scan_buffer(base: *const libc::c_char, size: YySize, yyscanner: YyScan) -> YyBufferState;
    pub fn yy_scan_bytes(base: *const libc::c_char, len: libc::c_int, yyscanner: YyScan) -> YyBufferState;
    pub fn yylex(yyscanner: YyScan) -> YyToken;
    pub fn yyget_text(yyscanner: YyScan) -> *mut libc::c_char;
    pub fn yy_delete_buffer(b: YyBufferState, yyscanner: YyScan);
    pub fn yylex_destroy(yyscanner: YyScan) -> libc::c_int;
}
