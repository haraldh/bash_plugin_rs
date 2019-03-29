#!/bin/bash
set -x

unset LIB

if [[ $(uname -s) == Darwin ]]; then
    DYN_EXT=dylib
else
    DYN_EXT=so
fi

LIB=$(ls -1 -t ./target/debug/deps/libhello*.${DYN_EXT} 2>/dev/null | { read a; echo $a; })

[[ $LIB ]] || exit 1

# dlopen the log builtin bash module
enable -f "$LIB" hello

help hello

hello arg1 arg2 arg3

[[ $(hello) == "Hello World!" ]]
