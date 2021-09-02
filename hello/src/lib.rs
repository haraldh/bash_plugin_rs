#![allow(non_upper_case_globals)]

//! Example usage of bash_plugin_rs

use bash_plugin_rs::{argv_list, builtin, word_list, BUILTIN_ENABLED};
use std::ffi::CStr;

pub fn hello_main(argv: &[&CStr]) -> i32 {
    if !argv.is_empty() {
        println!("Hello World! {:#?}", argv);
    } else {
        println!("Hello World!");
    }
    0
}

/// # Safety
/// All pointers in `word_list` must be valid.
#[no_mangle]
pub unsafe extern "C" fn hello_func(word_list: *mut word_list) -> i32 {
    hello_main(&argv_list(word_list))
}

#[no_mangle]
pub static mut hello_struct: builtin = builtin {
    name: b"hello\0" as *const u8,
    function: Some(hello_func),
    flags: BUILTIN_ENABLED as i32,
    long_doc: &mut [
        b"Print Hello World.\0" as *const u8,
        b"\0" as *const u8,
        b"Blah, Blah\0" as *const u8,
        b"\0" as *const u8,
        b"Usage:\0" as *const u8,
        b"\0" as *const u8,
        b"   $ hello \0" as *const u8,
        b"\0" as *const u8,
        0 as *const u8,
    ] as *mut _ as *const *const u8,
    short_doc: b"Prints Hello World\0" as *const u8,
    handle: 0 as *mut u8,
};
