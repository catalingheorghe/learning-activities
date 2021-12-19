### Rust in 3 weeks (safari online)

Note: week 1 was the foundamentals part, covered by standalone training crash
course in rust.

**Idiomatic** rust - rustfmt, cargo clippy.


Cargo **doc** - options no-deps, open. Marker is `///`. Always starts with a
description and then can have sections in comments. A documentation block
comment is `/** */`. 

An inner documentation comment is `//!` - e.g.: for libraries and modules. Block
comment for inner: `/*! */`.


Publishing on **crates.io** is permanent. Github account, access token ->
`cargo login`. Cargo.toml settings. Cargo publish. Website docs.rs automatically
gets populated with the generated documentation. Currently there is a flat
namespace.

You can set up a private crates registry.


**Closures** - annonymous function - `|params| expr` - that can be passed
around. Automatically borrows a reference to values that it uses. Can also move
variables into itself and take ownership `move || { println!("{}", s}; }`. To
pass closures to or from functions, see traits from `std::ops` Fn... .

Iterators - IntoIterator trait, implements `into_iter()`, which takes ownership
of the collection. So a for loop onto an iterator moves the value. But with
functional programming:

```
let v = vec![6, 7, 8];
v.into_iter.for_each(|num| println!("{}", num));
// faster than for loops
// can use Iterator Adaptors (map, filter)
```

Iterator Adaptors are executed only when there is also an Iterator Consumer.
Example, `for_each`, `sum`. They use generics heavily, so sometimes the type
must be specified with the turbofish operator `.sum::<i32>()`.

Commonly used iterator consumer - `collect()`, which creates a new collection
`collect::<Vec<_>>()`.

There are also iterators that takes immutable or mutable references - `iter()`,
`iter_mut()`.

syntatic sugar
```
v.into_iter()   for _ in v
v.iter()        for _ in &v
v.iter_mut()    for _ in &mut v
```

Removing (and returning) items from a collection, but leaving collection in
place: `drain()`. For a Vector it takes a range (`..` - full range), for a
HashMap it covers all.



