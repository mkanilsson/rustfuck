#!/usr/bin/bash
# set -e

echo "Compiling $1.S -> $1.o"
nasm -f elf64 "$1.S" -o "$1.o" || exit 1

echo "linking $1.o -> $1"
ld "$1.o" -o "$1" || exit 2

./$1

echo "$?"
