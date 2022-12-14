#!/bin/sh

set -xe

clang -Wall -Wextra -lSDL2 -o main.out main.c \
    src/qrcode.c src/pattern.c src/utils.c \
    src/information.c src/encoding.c src/array.c \
    src/log_antilog.c src/polynomial.c src/mask.c \
    src/logger.c src/gui.c
