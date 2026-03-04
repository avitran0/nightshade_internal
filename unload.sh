#!/bin/bash

gdb -p $(pidof csgo_linux64) -n -q -batch \
  -ex "set \$library = ((void*(*)(char*, int)) dlopen)(\"$PWD/target/release/libnightshade.so\", 6)" \
  -ex "set \$dlclose = (int(*)(void*)) dlclose" \
  -ex "call \$dlclose(\$library)" \
  -ex "call \$dlclose(\$library)" > /dev/null 2>&1
