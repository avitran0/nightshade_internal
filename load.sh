#!/bin/bash

./build.sh

sudo gdb -batch-silent -p $(pidof csgo_linux64) -ex "call (void*)dlopen(\"$PWD/build/libnightshade.so\", 2)" > /dev/null 2>&1
