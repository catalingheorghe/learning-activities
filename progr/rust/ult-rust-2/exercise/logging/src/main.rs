// START IN lib.rs!!!

use frogger::Frog;

fn main() {
    env_logger::init();

    // 9. Run this program with `cargo run` and take a look at the default output.
    // - Now run it again with an explicit log level, like `RUST_LOG=info cargo run`
    // - or `RUST_LOG=trace cargo run`
    let mut skippy = Frog::new();
    skippy.oac();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();

    // Challenge: Go back to lib.rs and set the `target: ` argument for each logging call to be the
    // path to the function.  For example, `Frog::new`
}
