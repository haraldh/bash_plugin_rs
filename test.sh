#!/bin/bash
set -x

unset LIB

if [[ $(uname -s) == Darwin ]]; then
    DYN_EXT=dylib
else
    DYN_EXT=so
fi

LIB_RELEASE=./target/release/examples/libhello.${DYN_EXT}
LIB_DEBUG=./target/debug/examples/libhello.${DYN_EXT}

if [[ -f $LIB_RELEASE ]] && [[ -f $LIB_DEBUG ]]; then
    if [[ $LIB_RELEASE -nt $LIB_DEBUG ]]; then
        LIB=$LIB_RELEASE
    else
        LIB=$LIB_DEBUG
    fi
elif [[ -f $LIB_RELEASE ]]; then
    LIB=$LIB_RELEASE
elif [[ -f $LIB_DEBUG ]]; then
    LIB=$LIB_DEBUG
fi

[[ $LIB ]] || exit 1

# dlopen the log builtin bash module
enable -f "$LIB" hello

help hello

hello

[[ $(hello) == "Hello World!" ]]
