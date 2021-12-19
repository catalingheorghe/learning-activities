use std::panic;

// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience:
// dyson.png and pens.png Feel free to use them or provide (or generate) your
// own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here:
// https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run
//         *noticeably* faster if you run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
//         message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a
//     number");

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" | "rotate" | "brighten" | "invert" | "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            match subcommand.as_str() {
                "blur" => { blur(infile, outfile) }
                "rotate" => { rotate(infile, outfile) }
                "brighten" => { brighten(infile, outfile) }
                "invert" => { invert(infile, outfile) }
                "grayscale" => { grayscale(infile, outfile) }
                _ => { panic!() }
            }
        }

        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("Failed parsing x.");
            let y: u32 = args.remove(0).parse().expect("Failed parsing y.");
            let w: u32 = args.remove(0).parse().expect("Failed parsing width.");
            let h: u32 = args.remove(0).parse().expect("Failed parsing height.");
            crop(infile, outfile, x, y, w, h);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" | "generate" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            match subcommand.as_str() {
                "fractal" => { fractal(outfile) }
                "generate" => { generate(outfile) }
                _ => { panic!() }
            }
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("    blur INFILE OUTFILE");
    println!("    brighten INFILE OUTFILE");
    println!("    crop INFILE OUTFILE x y width height (top left)");
    println!("    rotate INFILE OUTFILE");
    println!("    invert INFILE OUTFILE");
    println!("    grayscale INFILE OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String) {
    // Here's how you open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(2.0);
    // Here's how you save an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(30);
    img2.save(outfile).expect("Failed writing OUTFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
}

fn crop(infile: String, outfile: String, 
        x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.rotate90();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    // Set the image to some solid color. -- see fractal() for an example
    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.
    // Challenge 2: Generate something more interesting!

    let (width, height) = (800, 800);
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (red, mut green, blue) = (30, 30, 30);
        // this is the modulus operator, introduced in 1.38.0
        if y != 0 && x.rem_euclid(y) == 0 { 
            green = 100;
        }
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).expect("Failed writing OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
