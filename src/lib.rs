#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::os::raw::{c_void};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
pub struct builtin {
    pub name: *const u8,
    pub function: sh_builtin_func_t,
    pub flags: ::std::os::raw::c_int,
    pub long_doc: *const *const u8,
    pub short_doc: *const u8,
    pub handle: *mut u8,
}

unsafe impl Sync for builtin {}

pub fn argv_list<'a>(word_list: &'a mut WORD_LIST) -> Vec<&CStr> {
    let mut argv = Vec::<&CStr>::new();
    let mut word_list: *mut WORD_LIST = word_list;
    while (word_list as *mut _ as *mut c_void) != std::ptr::null_mut() {
        argv.push(unsafe {
            assert_ne!(std::ptr::null_mut(), (*(*word_list).word).word);
            CStr::from_ptr((*(*word_list).word).word)
        });
        unsafe {
            word_list = (*word_list).next;
        }
    }
    argv
}

#[test]
fn bindgen_test_layout_builtin() {
    assert_eq!(
        ::std::mem::size_of::<builtin>(),
        48usize,
        concat!("Size of: ", stringify!(builtin))
    );
    assert_eq!(
        ::std::mem::align_of::<builtin>(),
        8usize,
        concat!("Alignment of ", stringify!(builtin))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<builtin>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(builtin),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<builtin>())).function as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(builtin),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<builtin>())).flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(builtin),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<builtin>())).long_doc as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(builtin),
            "::",
            stringify!(long_doc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<builtin>())).short_doc as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(builtin),
            "::",
            stringify!(short_doc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<builtin>())).handle as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(builtin),
            "::",
            stringify!(handle)
        )
    );
}
