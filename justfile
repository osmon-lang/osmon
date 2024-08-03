#!/usr/bin/env just --justfile

release:
  cargo build --release    

lint:
  cargo clippy

format:
  cargo fmt

init:
    cargo build --release
    cd lib && gcc -c -o osmon_cstd.o osmon_cstd.c
    cd lib && gcc -shared -o libosmonstd.so osmon_cstd.o
    cp ./lib/libosmonstd.so ./std
    cp -R ./std ./stage_1/src/

# Self host
selfhost-init:
    cp -R ./std ./stage_1/src/

selfhost-all:
    cd stage_1 && ../target/release/osmon --backend gccjit src/main.osmx -o osmc

selfhost-run:
    cd stage_1 && ../target/release/osmon src/main.osmx --jit

# Standard library build
std-compile:
    cd lib
    gcc -c -o osmon_cstd.o osmon_cstd.c
    gcc -shared -o libosmonstd.so osmon_cstd.o

std-install:
    cp ./lib/libosmonstd.so ./std