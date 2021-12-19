// Silence some warnings so they don't distract from the exercise.
//#![allow(dead_code, unused_mut, unused_variables)]

// 6 types of strings in rust, but the most common String and &str
//   String is mutable, has a capacity, besides the content and length
//   &str is a borrowed slice string can't be changed
//
// strings in rust are utf-8 always
//   a unicode grapheme can have multiple scalars
//   and a scalar can have multiple bytes
//   => indexing [3] is not allowed in rust since it can't guarantee
//      constant speed
//   you need to get an iterator and then use nth() on that

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    //     cargo run apple banana
    // ...produces the equivalent of
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // note that if is an expression not a statement
        //
        // a = if a < 3 {
        //    "trei"
        //    };
        if arg == "sum" { // note that we are compartin a String with a &str here
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;

    // for in a range [7..24)
    //   or 7..=23 [7..23]
    for i in 7..24 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    // condition must evaluate to bool - no type cohersion
    while x <= 500 {
        x = x * 2;
        count = count + 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    // print arg 8 times
    let mut count = 0;

    'times8: loop {
        if count < 8 {
            print!("{} ", arg);
            count = count + 1;
        } else {
            break 'times8;
        }
    }
    // the times8 is a label for a loop, useful for nested loops and breaking
    // out of them; here - it is just for show

    println!(); // This will output just a newline at the end for cleanliness.
}
