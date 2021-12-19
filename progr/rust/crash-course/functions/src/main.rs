// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // in rust, all variables must be initialized at compile time
    // shadowing variable can be even done in the same scope
    //   let meme = "whatever";
    //   let meme = make_image(meme);
    // useful for data transformation

    {
        let area = area_of(width, height);
        println!("Area is {}", area);
        // area's scope ends here
    }

    println!("Volume is {}", volume(width, height, depth));
    // the ! is a macro, which is the only way you can call with variadic args
}

// - fn is pronounced "fun"
// - style guide - snake case for names
//     - there is only one styleguide, the official one
//     - rstfmt is also integrated into cargo - it formats the code accordingly
// - can be defined anywhere in the file
// - return can be `return true;` or if last statement `true`
//     - idiomatic way: use the "tail expression"
fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
