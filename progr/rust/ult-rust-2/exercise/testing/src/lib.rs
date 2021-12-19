// Unit Tests
// ==========
//
// submodule inline "test" at the bottom of file (convention)
// to not ship the test code in library '#[cfg(test)]'
//   config controls conditional compilation
//
// in the test module we can use * to get everythin into scope with
// 'use super::*;'
//
// attribute '#[test]' for functions to be ran as tests
//
// assert_eq!()
// assert_ne!()
// assert!(boolean value)
//
// attribute '#[should_panic]` will not stop the test suite
//
// tool:
// cargo test [test::bunny_results]
//
// unit tests can return a Result<(), ...>
//
// "crate"
// 1. crate = package (libraries, binaries)
// 2. crate = a library or a binary (usually used in rust language)
//
// Doc-tests - example in documentation that is ran as tests!
// (only for binaries, not libraries)
//
// # Example
//
// ```
// # use hello::snuggle;
// let bunnies = snuggle(5);
// assert_eq!(bunnies,40)
// ```
//
// Integration Tests
// =================
//
// tests directory - anyfile will be searched for tests to run
// test functions marked with same attribute
//
// Convention is to have small binary and put everything important in libraries.
// That means that binaries aren't usually tested. But you could do integration
// tests with std::process::Command (to run your binary and check output).
//
// Benchmarks
// ==========
//
// built-in not yet finished
// everybody uses Criterion
// 
// cargo.toml -> [dev-dependecies]
// a dependency can specify features as well
//
// benches/ directory (similat to integration tests)
//

pub fn sploosh(x: i32, y: i32, z: i32) -> i32 {
    match (x, y, z) {
        (x, _, _) if x < 0 => 99,
        (1, 2, 3) => 4,
        (5, 6, 7) => 3,
        (x, y, z) => x + y - z,
    }
}

pub fn splish(a: i32, b: i32) -> i32 {
    -a + 3 * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_sploosh_ok() {
        assert_eq!(sploosh(1, 2, 3), 4);
        assert_ne!(sploosh(5, 6, 7), 4);
        assert_eq!(sploosh(-3, 8, 9), 99);
    }

    #[test]
    fn is_splish_ok() {
        assert!(splish(100, 10) < 0);
        assert!(splish(40, 20) > 0);
        assert!(splish(9, 3) == 0);
    }
}

// Challenge: Create a benchmark that measures the speed of splish(8, 9, 10)
// - Speed up the implementation of splish(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started
