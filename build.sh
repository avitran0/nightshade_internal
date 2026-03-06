#!/bin/bash

mkdir -p build
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release -G Ninja
cmake --build build
