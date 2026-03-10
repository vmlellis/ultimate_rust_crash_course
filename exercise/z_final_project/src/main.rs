// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "imgtools",
    about = "Image processing tool (use .png extensions on filenames)",
    arg_required_else_help = true,
    flatten_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
struct BlurArgs {
    infile: String,
    outfile: String,
    #[arg(short, long, default_value_t = 2.0)]
    amount: f32,
}

#[derive(Parser)]
struct BrightenArgs {
    infile: String,
    outfile: String,
    #[arg(short, long, default_value_t = 10)]
    amount: i32,
}

#[derive(Parser)]
struct CropArgs {
    infile: String,
    outfile: String,
    #[arg(long)]
    x: u32,
    #[arg(long)]
    y: u32,
    #[arg(long)]
    width: u32,
    #[arg(long)]
    height: u32,
}

#[derive(Parser)]
struct RotateArgs {
    infile: String,
    outfile: String,
    #[arg(short, long, value_parser = ["90", "180", "270"])]
    degrees: String,
}

#[derive(Parser)]
struct FractalArgs {
    outfile: String,
    #[arg(long, default_value_t = 800)]
    width: u32,
    #[arg(long, default_value_t = 800)]
    height: u32,
    #[arg(long, default_value_t = -0.4)]
    re: f32,
    #[arg(long, default_value_t = 0.6)]
    im: f32,
    #[arg(long, default_value_t = 3.0)]
    scale: f32,
    #[arg(long, default_value_t = 1.5)]
    offset: f32,
    #[arg(long, default_value_t = 255)]
    max_iter: u8,
    #[arg(long, default_value_t = 0.3)]
    bg_intensity: f32,
}

#[derive(Parser)]
struct GenerateArgs {
    outfile: String,
    #[arg(long, default_value_t = 800)]
    width: u32,
    #[arg(long, default_value_t = 800)]
    height: u32,
    #[arg(long, default_value_t = 255)]
    red: u8,
    #[arg(long, default_value_t = 0)]
    green: u8,
    #[arg(long, default_value_t = 0)]
    blue: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Apply gaussian blur to an image
    Blur(BlurArgs),

    /// Crop an image
    Crop(CropArgs),

    /// Brighten or darken an image
    Brighten(BrightenArgs),

    /// Rotate an image (90, 180 or 270 degrees)
    Rotate(RotateArgs),

    /// Invert the colors of an image
    Invert {
        infile: String,
        outfile: String,
    },

    /// Convert an image to grayscale
    Grayscale {
        infile: String,
        outfile: String,
    },

    /// Generate a fractal image
    Fractal(FractalArgs),


    /// Generate a custom image
    Generate(GenerateArgs),
}

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let cli = Cli::parse();
    match cli.command {
        Commands::Blur(args) => blur(args),
        Commands::Brighten(args) => brighten(args),
        Commands::Crop(args) => crop(args),
        Commands::Rotate(args) => rotate(args),
        Commands::Invert { infile, outfile } => invert(infile, outfile),
        Commands::Grayscale { infile, outfile } => grayscale(infile, outfile),
        Commands::Fractal(args) => fractal(args),
        Commands::Generate(args) => generate(args),
    }
}

fn blur(args: BlurArgs) {
    let img = image::open(args.infile).expect("Failed to open INFILE.");
    let img2 = img.blur(args.amount);
    img2.save(args.outfile).expect("Failed writing OUTFILE.");
}

fn brighten(args: BrightenArgs) {
    let img = image::open(args.infile).expect("Failed to open INFILE.");

    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.
    let img2 = img.brighten(args.amount);
    img2.save(args.outfile).expect("Failed writing OUTFILE.");
}

fn crop(args: CropArgs) {
    let img = image::open(args.infile).expect("Failed to open INFILE.");
    let img2 = img.crop_imm(args.x, args.y, args.width, args.height);
    img2.save(args.outfile).expect("Failed writing OUTFILE.");
}

fn rotate(args: RotateArgs) {
    let img = image::open(args.infile).expect("Failed to open INFILE.");
    let img2 = match args.degrees.as_str() {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => unreachable!("only 90, 180, or 270 reach here"),
    };
    img2.save(args.outfile).expect("Failed writing OUTFILE.");
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

fn generate(args: GenerateArgs) {
    // Create an ImageBuffer -- see fractal() for an example
    let mut imgbuf = image::ImageBuffer::new(args.width, args.height);

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = args.red.wrapping_add((x as f32 * 0.1) as u8);
        let g = args.green.wrapping_add((y as f32 * 0.1) as u8);
        let b = args.blue.wrapping_add(((x + y) as f32 * 0.05) as u8);
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save(args.outfile).expect("Failed writing OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(args: FractalArgs) {
    let width = args.width;
    let height = args.height;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = args.scale / width as f32;
    let scale_y = args.scale / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (args.bg_intensity * x as f32) as u8;
        let blue = (args.bg_intensity * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - args.offset;
        let cy = x as f32 * scale_y - args.offset;

        let c = num_complex::Complex::new(args.re, args.im);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < args.max_iter && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(args.outfile).unwrap();
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
