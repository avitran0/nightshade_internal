#!/bin/bash

cargo build --release
sudo gdb -batch-silent -p $(pidof csgo_linux64) -ex "call (void*)dlopen(\"$PWD/target/release/libnightshade.so\", 2)" > /dev/null 2>&1
