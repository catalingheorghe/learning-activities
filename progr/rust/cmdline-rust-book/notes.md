https://github.com/kyclark/command-line-rust

### Project structure and basics

```
dir/
  bin/       <-- bin targets (cargo run --bin true)
    true.rs
  src/
    main.rs
  tests/     <-- integration tests
    cli.rs
```

Rustc is the compiler, but cargo can help with creating a project structure,
then building, running, etc. Simplest way to create a new project: 
`cargo new name`.

Cargo.toml holds the project name, version (semantic versioning), and
dependencies, including dev-dependencies.

A project can have multiple binaries, not only the main one - bin/ subfolder,
for example.

Run a project
 - `cargo run`
 - `cargo run -- -n arg1 arg2` - pass args
 - `cargo run --release` - not debug version (default)
 - `cargo run --bin namebin` - run alternative binaries

To run the tests 
 - `cargo test`. 
    They can run in any order (--test-threads=1 to stop that).
	Test functions are marked with `#[test]`. 
 - `cargo test dies` - run all the tests that contain "dies" in name

Only build
 - `cargo build`

Clean
 - `cargo clean` (removes target directory)

**Glossary** for various terms, eg target:
https://doc.rust-lang.org/cargo/appendix/glossary.html#target

**Documentation** - docs.rs

### Variable, types

Names

 - starting with `_` tells rustc I don't intend to use the variable

Types

 - unit type - `()`, when there is no other meaningful value that could be
   returned. `main` returns this.

 - String - a string of characters.

 - str - a valid UTF-8 string; appropiate for literal strings;

 - bool - true or false.

 - Option - either None or `Some<T>`, where T is any type.
   - the `unwrap` function takes the value out of `Some<T>`, panic if `None`

 - Result - either success, with a type, or Err.
   - `unwrap` takes value in Ok, or panic
   - `?` either unpack an ok value, or propagate the `Err` to the caller

 - Vector - contiguous growable array size; same types.

 - std::slice - similar to vectors, but can't be resized after creation.

Type aliases

Example
`type TestResult = Result<(), Box<dyn std::error::Error>>;`
a specific type of Result, for which OK part will only be the unit type, and the
Err part can hold anything implementing that trait, to which calls can be
dispatched dynamically and outcomes stored on the heap.

Mutability and borrowing

 - variables are immutable by default (`let mut ...`)
 - a '&' shows the intent to borrow

### Control flow

return- there is a `return` keyword, but it is idiomatic to omit the last
semicolon to return that

`if` is an expression, not a statement. It returns a value.
`let value = if winter { "cold" } else { "warm" };`
Without an else, returns unit type.



### Traits

A trait is a way to define behavior of an object.

### Crates

General notes about crates

Source code downloaded -> `.cargo`. Build artifacts -> `target/debug/deps` (each
program is built in a separate directory; dependencies can be different
versions). Python offers virtual envs to solve this. Downside is target
directory in rust will be large.

Importing
 - `use clap:{App, Arg};` - import two structs from clap crate

#### Useful crates

`std::process` - handle external processes

`std::env` - interact with environment
The `Args` struct, returned by `args()`, contains cmd line arguments, for ex.

`std::fs` - filesystem access
Example - `fs::read_to_string()` - note that it will read ALL the file.

`clap` - command line arguments parser

`assert_cmd` - for testing your CLI program like a user

`predicates` - 

### General programming

**Testing**

More than the act of testing, the act of designing tests is one of the best bug
preventers known. The thinking that must be done to create a useful test can
discover and eliminate bugs before they are coded—indeed, test-design thinking
can discover and eliminate bugs at every stage in the creation of software,
from conception to specification, to design, coding, and the rest.

Boris Beizer, Software Testing Techniques (Van Nostrand Reinhold)

(from chapter 1)

Broad categories

 - Inside-out or unit tests - tests for functions
 - Outside-in or integration intest - run as the user might run the program

(from chapter 2)

Even with a simple tool, there is a decent amount of "cyclomatic complexity" -
various ways all parameters can be combined.

