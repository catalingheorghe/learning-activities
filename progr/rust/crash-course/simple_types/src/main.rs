// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]

use ding_machine::print_difference;
use ding_machine::print_array;
use ding_machine::ding;
use ding_machine::on_off;
use ding_machine::print_distance;

// more common
//   use ding_machine::{print_difference, print_array};
// wildcard usage
//   use ding_machine::*;
//   not idiomatic, not recommended in general
//   only if a library defines a prelude submodule
//     use ding_machine::prelude::*;
//   prelude is just a module called prelude
//   std library also has a prelude that is always imported

// note that it throws warnings even if you import something that you don't use

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);

    // tuples accessed with dot notation; they can hold different types
    //   - max arity (num of elements) = 12
    print_difference(coords.0, coords.1 );

    // arrays are fixed length at compile time (Vec for dynamic arrays)
    //   - same type; number of elements after ';'
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0); // true

    print_distance(coords_arr[0], coords_arr[1]);
}
