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
then building, running, etc.

Cargo.toml holds the project name, version (semantic versioning), and
dependencies, including dev-dependencies.

A project can have multiple binaries, not only the main one - bin/ subfolder,
for example.

To run the tests - `cargo test`. They can run in any order (--test-threads=1 to
stop that). Test functions are marked with `#[test]`.

**Glossary** for various terms, eg target:
https://doc.rust-lang.org/cargo/appendix/glossary.html#target

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




