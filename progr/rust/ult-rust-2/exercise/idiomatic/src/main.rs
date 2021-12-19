// cargo fmt will format to the proper style guide
//   will not add newlines
//   can be configured in vscode to "format on save"
//
// cargo clippy will build the code and do a lot of linting on it
//

// Challenge: Clippy doesn't find *everything*. What else would you change to make this code better?

use core::f32::consts::PI;

fn count_to_5() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > PI as i32 + 1 {
            break;
        }
    }
    counter
}

fn main() {
    println!("I can count to {}", count_to_5());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
