# bfc-rs
Yet another [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to C transpiler.

## Before starting
`bfc-rs` has only 1 function, transpiling Brainfuck to c.

**It does**
* allocated a statically sized tape, 30 000 in length (this will be dynamic in future)
* write the transpiled c code to a c file

**It does not**
* optimise the code
* check for tape underflow and overflow
* check for cell value overflow

## Usage
 ```shell
 bfc-rs [OPTIONS] <input>

OPTIONS:
	-o, --out <file>		output file [default: ./out.c]

ARGS:
	<input>   input file
 ```
### Example
```shell
$ bfc-rs hello_world.bf -o hello_world.c
$ gcc ./hello_world.c -o out && ./out
Hello World! 
```

With cargo
```shell 
bfc-rs $ cargo run --release -- hello_world.bf -o hello_world.c
bfc-rs $ gcc ./hello_world.c -o out && ./out
Hello World!
```
