# Rustfuck

A brainfuck to x86_64 intel assembly Compiler written in rust.

Currently, it required libc and needs to be compiled and linked with `as` and `gcc`.

## Usage

```sh
Brainfuck to x86_64 intel assembly Compiler

Usage: rustfuck [OPTIONS] <INPUT_PATH>

Arguments:
  <INPUT_PATH>  Brainfuck source file

Options:
  -o, --output-path <OUTPUT_PATH>  Output path
  -O                               Enable optimizations
  -S, --assembly                   Output generated assembly
      --keep-files                 Keep intermediate files
      --ast                        Print generated AST
  -h, --help                       Print help
  -V, --version                    Print version
```

### Compile brainfuck to executable

```sh
rustfuck brainfucks/hello_world.bf
```

this will output an executable called `a.out` 


```sh
rustfuck brainfucks/hello_world.bf --keep-files
```

this will output an executable called `a.out`, an assembly file called `a.S` and an object file called `a.o`.


```sh
rustfuck brainfucks/hello_world.bf -o hello_world
```

this will output an executable called `hello_world`


```sh
rustfuck brainfucks/hello_world.bf -o hello_world --keep-files
```

this will output an executable called `hello_world`, an assembly file call `hello_world.S` and an object file called `hello_world.o`

### Compile brainfuck to x86_64 assembly

```sh
rustfuck brainfucks/hello_world.bf -S
```

this will output an assembly file called `a.S` 


```sh
rustfuck brainfucks/hello_world.bf -S -o hello_world
```

this will output an assembly file called `hello_world`


```sh
rustfuck brainfucks/hello_world.bf -o hello_world.s
```

this will output an assembly file called `hello_world.s`

### Dump AST

```sh
rustfuck brainfucks/hello_world.bf --ast
```

this will dump the generated AST to stdout

## TODO

* Better error handling
* Unit tests and CI
* Use linux syscalls instead of libc
* Remove `as` and `gcc` dependancies and start using `nasm` or `fasm`
* Allow outputting as object file
