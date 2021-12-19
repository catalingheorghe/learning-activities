// Yes, yes, we know. It's an exercise, compiler, we want it that way!
#[allow(unused_mut)]

fn main() {
    // closure that returns a number squared
    let square = |x| x * x;
    println!("5 squared is {}", square(5));

    // map takes a closure and applies it to each item, creating another iterator
    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    pairs
        .into_iter()
        .map(|(x, y)| (x + 1, y))
        .for_each(|t| println!("{:?}", t));
    // the below does NOT compile, because pairs was moved into 
    //   `into_iter(self) -> Self::IntoIter`
    // println!("pairs length: {}", pairs.len());

    // iterate over a mutable collection with mutable references and modify the items
    let mut numbers = vec![1, 2, 3, 4];
    for x in numbers.iter_mut() {
        // x is a mutable reference
        *x *= 3;
    }
    // syntactic sugar only, same thing
    for x in &mut numbers {
        *x *= 3;
    }
    println!("{:?}", numbers); // should print [3, 6, 9, 12]

    // 4. Uncomment the code below.  Take the vector of words and
    // - Convert the vector into an iterator with .into_iter()
    // - Use .filter() to remove any word that contains the letter "h" -- use .contains()
    // - Use .map() to convert all the words to uppercase -- use .to_uppercase()
    // - Use .collect() to put the transformed words back into a vector
    //
    // Hint: .to_uppercase() is a method on `str` which returns a String
    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed: Vec<_> = words  // this will be a Vec<String> because of to_uppercase
        .into_iter()
        .filter(|s| !s.contains("h"))
        .map(|s| s.to_uppercase())
        .collect();
    println!("Transformed: {:?}", transformed);

    // Challenge:
    //
    // - Rewrite the code in #2 as a for loop
    let pairs2 = vec![(0, 1), (2, 3), (4, 5)];
    for pair in pairs2.into_iter() {
        let t = (pair.0 + 1, pair.1);
        println!("{:?}", t);
    }

    // - Rewrite the code in #3 in functional style (without a for loop).
    // Hint: There are multiple
    // ways to accomplish this, but they all end with an iterator consumer.
    let mut numbers2 = vec![1, 2, 3, 4];
    numbers2.iter_mut().for_each(|x| *x *= 3);
    println!("{:?}", numbers2);
}
