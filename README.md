# POETIC

Rust library to parse and interpret [poetic](https://mcaweb.matc.edu/winslojr/vicom128/final/index.html) source code

It supports all of the instructions and handles digit conversion accordingly, with the only difference being that it dynamically allocates more memory if needed instead of 30.000 bytes. This means memory pointer wrapping at the "end" is not possible. Maybe in the future through a option.
https://mcaweb.matc.edu/winslojr/vicom128/final/tutorial/index.html

## Tests

There are tests for the parser and interpreter which you can run with:

```Rust
cargo test
```

## Usage

It will first parse the string source input and parse it to intermediate opcode number representation.

```Rust
    let mut buf = fs::read_to_string("input.ptc").unwrap();

    let intermediate = Parser::parse_intermediate(&buf);
```

Then it will convert the intermediate representation to the instructions.

```Rust
    let instructions = Parser::parse_instructions(&intermediate);
```

These instructions can then be executed with the interpreter

```Rust
    let mut interpreter = Interpreter::new(code);
    interpreter.run();
```

You can let the interpreter just fully execute it with the `run` method or step through every instruction with the `step` method.

## Example

An example usage can be found at https://github.com/mztikk/poetic_interpreter which is a cli application that will take a poetic source file and execute it.
