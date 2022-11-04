### Rust - ultimate crash course (safari learning)

Modern approach to a low level language. Tries to solve a lot of problems that
can pop up in systems programming at compile time.

Rust compiler throws a lot of warnings, with good explanations. And you have to
explcitly ignore them in the code. There is even a warning about not doing
something with the result of a function (when it returns a Result enum).

Is type safe, aka no undefined behaviour, at compile time (other higher
languages are type safe at runtime). C went for speed by allowing compilers to
do whatever they want when accessing beyond an array, for ex.

Variables are immutable by default. It even warns if it is mutable but not
modified.

The true novelty in the language, with which it hopes to address memory safety
issues is ownership. A value can be owned by a single owner at a given time.
Passing the value transfers ownership. When owner is out of scope, it is
destroyed. Can be borrowed via references. There can be multiple immutable
references, but only one mutable at a given time.

Handles objects and inheritance by "traits", which represent behaviours that
objects (structs) can implement. There is no official constructor, but defacto,
the "new" name is used for such a method.

Enums are also interesting, being a sort of union, capable of storing different
variants, but each variant can also have a value. And enums can have
implementation methods as well.

Tooling (cargo) tries to bundle in everything - toolchain management, compiling,
linting, styling, documentation, testing, package mamagement.
