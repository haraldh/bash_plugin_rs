#![allow(non_upper_case_globals)]

use bash_plugin_rs::{builtin, BUILTIN_ENABLED};

#[no_mangle]
pub unsafe extern "C" fn hello_func(_word_list: *mut bash_plugin_rs::word_list) -> i32 {
    println!("Hello World!");
    return 0;
}

#[no_mangle]
pub static mut hello_struct: builtin = builtin {
    name: b"hello\0" as *const u8,
    function: Some(hello_func),
    flags: BUILTIN_ENABLED as i32,
    long_doc: unsafe { &hello_usage as *const _ as *const u8 },
    short_doc: b"Prints Hello World\0" as *const u8,
    handle: 0 as *mut u8,
};

#[no_mangle]
pub static mut hello_usage: [*const u8; 9] = [
    b"Print Hello World.\0" as *const u8,
    b"\0" as *const u8,
    b"Blah, Blah\0" as *const u8,
    b"\0" as *const u8,
    b"Usage:\0" as *const u8,
    b"\0" as *const u8,
    b"   $ hello \0" as *const u8,
    b"\0" as *const u8,
    0 as *const u8,
];

