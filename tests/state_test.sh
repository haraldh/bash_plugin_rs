#!/bin/bash
set -x

unset LIB

if [[ $(uname -s) == Darwin ]]; then
    DYN_EXT=dylib
else
    DYN_EXT=so
fi

LIB=$(ls -1 -t ./target/debug/deps/libstate*.${DYN_EXT} 2>/dev/null | { read a; echo $a; })

[[ $LIB ]] || exit 1

# dlopen the log builtin bash module
enable -f "$LIB" state

help state

state

[[ $(state) == 1 ]] || exit 1
[[ $(state) == 1 ]] || exit 1

state
[[ $(state) == 2 ]] || exit 1

state
[[ $(state) == 3 ]] || exit 1
