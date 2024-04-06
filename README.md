# Brainfuck interpreter
This is a simple brainfuck interpreter written in rust.

Tests in the `test` folder come from the [brainfuck.org](http://www.brainfuck.org/) website.

## Compilation
Simply run `cargo build --release` to build the interpreter, you will then find the executable in the target/release folder.
Alternatively, run `cargo run -- [OPTIONS]` to run the executable directly.

### Example runs

Simple execution
```
$ ./brainfuck -f examples/hello_world.b
Hello World!
Interpreted 906 instruction(s)
```

Run interactively
```
$ ./brainfuck -f examples/hello_world.b -i
> help
Commands are the following
- r | run                  : Run the rest of the program
- s | step [number]        : Run the next number of instruction (default is 1)
- q | quit                 : Quit the program
- p | print index[:amount] : Print the memory at index (default is 0) with amount (default is 1)
- o | output               : Print the current output
- h | help                 : Print the help message
- c | counter              : Print the number of instruction executed
- d | dp                   : Print the current data pointer
> print 0 10
0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
> step 100
> counter
Number of instruction executed: 100
> print 0 10
8, 0, 9, 13, 11, 4, 1, 0, 0, 0,
> 
```