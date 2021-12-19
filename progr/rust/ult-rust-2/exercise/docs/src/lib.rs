// run `cargo doc --no-deps --open` and take a look!

// module documentation - inner doc, not outer doc ('//!')

//! Library for handling pumpkins.
//!
//! A pumpkin is a cultivar of winter squash that is round with smooth, slightly ribbed skin, and
//! is most often deep yellow to orange in coloration. The thick shell contains the seeds and
//! pulp. The name is most commonly used for cultivars of Cucurbita pepo, but some cultivars of
//! Cucurbita maxima, C. argyrosperma, and C. moschata with similar appearance are also sometimes
//! called "pumpkins".
//!
//! ![pumpkin image](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)
// note: ![]() inlines the image, []() creates a link

// struct documentation

/// Big orange thing
///
/// # Recipes
/// Recipes will be coming soon.
pub struct Pumpkin {
    /// How round is it, a percentage from 0.0 to 1.0
    pub roundness: f32,
    /// How orange is it, on a scale from 8 to 27
    pub orangeness: i32,
}

// method documentation

impl Pumpkin {
    /// Takes a pumpkin and smashes it, consuming it
    ///
    /// If you smash a pumpkin, it can't be use for pie
    pub fn smash(self) {}
}

// can link to other items

/// Color reddish orange, see [Pumpkin::orangeness]
pub const BURNT_ORANGE: i32 = 13;

// --document-private-items

/// For internal use only. In fact, this documentation is so private that it
/// won't be generated. At least not by default. But if you pass the correct
/// option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}
