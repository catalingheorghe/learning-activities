// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

// A Shot enum to classify shots thrown at a target
//
// an enum is a datatype with variants
//  - a variant can hold data or not
//  - an enum can have also generic types
#[derive(Debug)]
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

// An enum can have an implementation, methods
//  - both methods specific to a shot (self)
//  - or associative methods (called with Shot::method_name()
impl Shot {
    // Here is a method for the `Shot` enum you just defined.
    // Convert a shot into points
    //  - use :: to go into the namespace of the enum
    //  - the 'match' must cover all variants of the enum (don't cate is '_')
    fn points(self) -> i32 {
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x)   => if x < 3.0 { 2 } else { 1 },
            Shot::Miss     => 0,
        }
    }

    // associative method - must be called with Shot::dummy_points()
    fn dummy_points() -> i32 {
        42
    }
}

// Some very common std enums are Option<T> and Result<T, E>
// The Result enum has a must_use annotation, so the compilere will warn if
// a function returns this, but the code does not do anything with the result.
//

// bring all identifier of Shot into the namespace
use Shot::*;

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // 2. For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for coord in arrow_coords {
        coord.print_description();

        let dist = coord.distance_from_center();
        let shot = if dist < 1.0 {
            Bullseye
        } else if dist < 5.0 {
            Hit(dist) // dist is a copy type so it is not moved
        } else {
            Miss
        };
        shots.push(shot);
    }

    // do a total of the point for each shot
    let mut total = 0;
    for shot in shots {
        total += shot.points();
        // note that at this point the value is actually moved, the points
        // function takes ownership; Why? Shot does not implement the Copy
        // trait, nor do we borrow it in point
        //
        // So, this will throw a compiler warning. Cool
        //
        // println!("shot: {:?}", shot)
    }

    println!("Final point total is: {:?}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
//  - this creates a Vec and returns it
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}

// std collection types include
//  - Vec
//  - HashMap
//  - LinkedList
//  - HashSet
//  - VecDeque
//  - BTreeMap
//  - BTreeSet
//  - BinaryHeap
//
//  optimized Vec collections exist for example only for integers
//  
