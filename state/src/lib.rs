#![allow(non_upper_case_globals)]

use bash_plugin_rs::{argv_list, builtin, word_list, BUILTIN_ENABLED};
use std::ffi::CStr;
use std::sync::{Arc, RwLock};

pub fn state_main(_argv: &[&CStr]) -> i32 {
    let state = State::current();

    println!("{}", state.counter());

    state.inc_counter();
    0
}

/// # Safety
/// All pointers in `word_list` must be valid.
#[no_mangle]
pub unsafe extern "C" fn state_func(word_list: *mut word_list) -> i32 {
    state_main(&argv_list(&mut *word_list))
}

#[derive(Default)]
struct StateInner {
    counter: u8,
}

struct State {
    inner: RwLock<StateInner>,
}

impl State {
    pub fn new() -> Arc<State> {
        Arc::new(State {
            inner: RwLock::new(Default::default()),
        })
    }
    pub fn current() -> Arc<State> {
        CURRENT_STATE.with(|c| c.clone())
    }
    pub fn counter(&self) -> u8 {
        self.inner.read().unwrap().counter
    }
    pub fn inc_counter(&self) {
        self.inner.write().unwrap().counter += 1;
    }
}

thread_local! {
    static CURRENT_STATE: Arc<State> = State::new();
}

#[no_mangle]
pub static mut state_struct: builtin = builtin {
    name: b"state\0" as *const u8,
    function: Some(state_func),
    flags: BUILTIN_ENABLED as i32,
    long_doc: &mut [
        b"Store some state.\0" as *const u8,
        b"\0" as *const u8,
        b"Blah, Blah\0" as *const u8,
        b"\0" as *const u8,
        b"Usage:\0" as *const u8,
        b"\0" as *const u8,
        b"   $ state new\0" as *const u8,
        b"\0" as *const u8,
        0 as *const u8,
    ] as *mut _ as *const *const u8,
    short_doc: b"Store some state\0" as *const u8,
    handle: 0 as *mut u8,
};
