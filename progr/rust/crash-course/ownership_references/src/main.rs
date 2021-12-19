// Silence some warnings so they don't distract from the exercise.
//#![allow(unused_mut)]

// ownership is the big differentiator of rust - tries to guarantee memory
// safety at compile time
//
//  1. every value in memory must have an owner
//  2. there must be only one owner at a time
//  3. when there is no more owner it is destroyed
//
// that means that if you assign something to another value, that is moved
// not copied. The clone() method is a way to a deep copy, for example
//
// for basic types, small types, there is an implicity Copy method (trait)
// that is defined so they are not moved, but copied (Copy can only be used
// for values that use only the stack, not the heap)
//
// and it also means that when you pass something as an argument to a function
// it is also moved, but that means you can't use it afterwards; you need to
// pass a reference, so that the function can borrow ownership to that value
//
//  - by default, references are immutable (can't modify the referenced)
//  - you can either have only immutable references OR one single mutable
//    reference to a value at the same time
//
fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // pass a reference to arg (immutable reference, by default)
    inspect(&arg);

    // pass a mutable reference to arg so that the function can change it
    change(&mut arg);
    println!("I have many {}", arg);

    // pass the ownership alltogether here; the value from arg will be moved
    // into the function scope, so it will not be available anymore
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.
    //
    println!("1 + 2 = {}, even via references", add(&1, &2));
}

fn inspect(s: &String) {
    // note that '.' does the derefencing as well
    // long way to write it would be '*s).ends_with("s")'
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

// note that he passed String must also be mutable, not only the reference
fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

// this does not borrow ownership, but takes it
fn eat(s: String) -> bool {
    // this evaluates to true or false, which is then returned
    s.starts_with("b") && s.contains("a")
}

fn add(a: &i32, b: &i32) -> i32 {
    *a + *b
}

