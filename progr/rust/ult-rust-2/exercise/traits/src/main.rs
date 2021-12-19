// A trait can be implemented by structs, enums, closure, functions.
//
// Common derivable traits
//  - Debug - Debug {:?}, Pretty {:#?}
//  - Clone - allows value to be cloned (clone())
//  - Copy  - type will be copied instead of moved
//      a type using heap at all, can't implement Copy
//      Copy is a subtrait of Clone, so you must #[derive(Clone, Copy)]
//
// Implement traits
// must bring Trait into scope (use) - some are in scope (std prelude)
//  - e.g.: Default for your type
//    (specify a value of a field, rest "struct update syntax" - def values)
//  - others: PartialEq / Eq, From / Into

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

impl From<Party> for Cake {
    fn from(party: Party) -> Self {
        party.cake
    }
}

// let's not consume the party to look at the cake
impl From<&Party> for Cake {
    fn from(party: &Party) -> Self {
        party.cake
    }
}

#[derive(Debug)]
pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}
// party can derive Debug since all its members implement (or derive) it

impl Default for Party {
    fn default() -> Self {
        Party {
            at_restaurant: true,
            num_people: 8,
            cake: Cake::Chocolate,
        }
    }
}

// as long as we have the same cake, same thing
impl PartialEq for Party {
    fn eq(&self, other: &Self) -> bool {
        // we must derive PartialEq for Cake in order to check on it
        self.cake == other.cake
    }
}

fn main() {
    let cake = Cake::Spice;
    admire_cake(cake);
    // cake has been moved to the admire_cake() function

    // To address this, we are deriving Clone and Copy for Cake as well,
    // so it will get copied, not moved around.
    match cake {
        Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
        Cake::MapleBacon => println!("Dreams do come true!"),
        Cake::Spice => println!("Great, let's spice it up!"),
    }

    println!("The default Party is\n{:#?}", Party::default());

    // create your own party with MapleBacon, but default values
    // "struct update syntax"
    let party = Party {
        cake: Cake::MapleBacon,
        ..Default::default()
    };
    println!("Yes! My party has my favorite {:?} cake!", party.cake);

    // PartialEq derived on Cake, and implemented on Party
    // two parties with same cake are the same
    let other_party = Party {
         at_restaurant: false,
         num_people: 235,
         cake: Cake::MapleBacon,
    };
    if party == other_party {
         println!("Your party is just like mine!");
    }

    // From<Party> for Cake
    //smell_cake(party);
    //println!("Do we still have a party here? {:#?}", party);
    // no we don't ... it's moved and consumed

    // but if we implement the From trait for the reference, then we can smell it
    // without consuming it
    smell_cake(&party);
    println!("Yum! I'm eating this cake: {:?}. Oops, I dropped it on the floor.", party.cake);
    drop(cake);
}

pub fn admire_cake(cake: Cake) {
    println!("What a nice {:?} cake! 🎂", cake);
}

// note the generics here - can pass in anything that can be transformed
// into a Cake (either a Party, or a reference to Party, or whatever)
pub fn smell_cake<T: Into<Cake>>(something: T) {
     println!("Hmm...something smells like a {:?} cake!", something.into());
}
