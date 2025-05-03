
# Branf*ck compiler

Language https://esolangs.org/wiki/Brainfuck

Compiles brainf*ck code into LLVM IR and then passes that into *clang* to make an executable.
It should be technically considered compiler.

Licensed under `GNU GENERAL PUBLIC LICENSE version-3`.

| command | description                                                                              |
|---------|------------------------------------------------------------------------------------------|
| `>`     | Moves the data pointer right by 1 element                                                |
| `<`     | Moves the data pointer left by 1 element                                                 |
| `+`     | Increases the value at the data pointer by 1                                             |
| `-`     | Decreases the value at the data pointer by 1                                             |
| `.`     | Prints the value at the data pointer interpreted as ASCII (equivalent to putchar)        |
| `,`     | Reads one character and save its ASCII value at the data pointer (equivalent to getchar) |
| `[`     | Starts a loop (jumps to the matching bracket if data pointer is zero)                    |
| `]`     | If the value at the data pointer is nonzero, jump back to the matching bracket           |
| `!`     | Prints the number at the data pointer                                                    |

## usage

requires `clang` and `libc` or equivalent C-runtime

```
Usage: bf [COMMAND]

Commands:
  compile  Compiles a source file
  repl     Takes user input and compiles that as source
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Compiles a source file
```
Usage: bf compile [OPTIONS] <source>

Arguments:
  <source>  Source file (file containing brainfck program)

Options:
  -o, --output <file>              Specifies output filename [default: out]
  -c, --cell-count <count>         Specifies how many cells should there be [default: 30000]
  -x, --do-not-compile             Does not execute 'clang' to compile llvm IR to executable
  -e, --emit-file <file>           Sets filename for emitted llvm IR
  -n, --override-new-line-as-null  Makes '\n'(0) be interpreted by Input command(',') as null(0)
  -h, --help                       Print help
```

### Takes user input and compiles that as source
```
Usage: bf repl [OPTIONS]

Options:
  -o, --output <file>              Specifies output filename [default: out]
  -c, --cell-count <count>         Specifies how many cells should there be [default: 30000]
  -x, --do-not-compile             Does not execute 'clang' to compile llvm IR to executable
  -e, --emit-file <file>           Sets filename for emitted llvm IR
  -n, --override-new-line-as-null  Makes '\n'(0) be interpreted by Input command(',') as null(0)
  -h, --help                       Print help
  -V, --version                    Print version
```

# interesting brainf*ck programs

- [tic tac toe](https://mitxela.com/projects/bf_tic_tac_toe)
- [square numbers from 0 to 10000.](https://brainfuck.org/squares.b)
- [Conway's Game of Life](https://brainfuck.org/life.b)
- [brainf*ck to C live translator](https://brainfuck.org/dbf2c.b)
- [computation the transcendental number e](https://brainfuck.org/e.b)
- [almost random](https://brainfuck.org/random.b)
- 