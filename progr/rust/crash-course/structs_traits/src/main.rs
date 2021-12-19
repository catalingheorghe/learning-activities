// traits are behaviours/actions that structs can implement; they are the Rust
// solution to complex inheritance problems
//
// structs don't have inheritance, but traits do
// if a struct implements a sub-trait, it must implement the methods for the
// parents as well
trait Bite {
    fn bite(self: &mut Self);
    // usually, by convetion, syntactic sugar for self only
    // fn bite(&mut self);
}

#[derive(Debug)] // to be able to use debug represantion {:?}
struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        if self.amount_left > 1 {
            self.amount_left -= 1;
        }
    }
}

fn bunny_nibbles<T: Bite>(food: &mut T) {
    food.bite();
    food.bite();
    food.bite();
    food.bite();
}

fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
