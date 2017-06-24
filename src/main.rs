//! # The Sierpinski Triangle
//!

extern crate image;
extern crate rand;
extern crate clap;

use rand::distributions::IndependentSample;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};

///
/// Generate the sierpinski triangle image
///
/// # Arguments
/// * `filename` - Path to the file to save image in
/// * `n` - Amount of iterations for the "Chaos Game" method of generation of
/// triangle
///
fn generate_sierpinski_tri<'a>(filename: &'a str,
                               img_width: u32,
                               img_height: u32,
                               n: u32,
                               clr: [u8; 4]) {
  use rand::distributions::Range as RandRange;
  use image::{ImageBuffer, ImageFormat, ImageRgba8, Rgba};
  use rand::thread_rng;

  // Create a new random number generator and some amount of random distributions
  let mut rng = thread_rng();
  let rand_starting_pnt_x_range = RandRange::new(0, img_width);
  let rand_starting_pnt_y_range = RandRange::new(0, img_height);
  let rand_corner_range = RandRange::new(0, 3);

  // Pick three corners
  let corners = [(0, img_height), (img_width, img_height), ((img_width / 2) as u32, 0)];

  // Select random starting point
  let mut pnt = (rand_starting_pnt_x_range.ind_sample(&mut rng),
                 rand_starting_pnt_y_range.ind_sample(&mut rng));

  // Create a new image buffer
  let mut img_buf: ImageBuffer<Rgba<u8>, _> = ImageBuffer::new(img_width, img_height);

  // Create a new pixel to use
  let pixel = Rgba(clr);

  // Generate the image
  for _ in 1..n {
    // Write a pixel
    img_buf.put_pixel(pnt.0, pnt.1, pixel);

    // Select random corner
    let rand_corner_num = rand_corner_range.ind_sample(&mut rng);
    let rand_corner = corners[rand_corner_num];

    // Calculate the position of the next point
    pnt.0 = (pnt.0 + rand_corner.0) / 2;
    pnt.1 = (pnt.1 + rand_corner.1) / 2;
  }

  // Open a new file and save the image
  let path = Path::new(filename);
  let mut file = File::create(path).unwrap();
  let _ = ImageRgba8(img_buf).save(&mut file, ImageFormat::PNG).unwrap();
}

///
/// Main function
///
fn main() {
  let matches = App::new("sierpinski-triangle-rs")
    .version("1.0")
    .author("Dmitry Guzeev <dmitry.guzeev@yahoo.com>")
    .about("Generates Sierpinski Triangle using Chaos Game algorithm")
    .arg(Arg::with_name("iter-num")
         .short("n")
         .long("iter-num")
         .help("Sets the number of iterations")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("output")
         .short("o")
         .long("output")
         .help("Sets the output file")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("verbose")
         .short("v")
         .long("verbose")
         .help("Sets verbosity"))
    .arg(Arg::with_name("img-width")
         .short("w")
         .long("img-width")
         .help("Sets the width of the output image")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("img-height")
         .short("h")
         .long("img-height")
         .help("Sets the height of the output image")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("red")
         .short("r")
         .long("img-red")
         .help("Sets the red amount in the color of output image")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("green")
         .short("g")
         .long("img-green")
         .help("Sets the green amount in the color of output image")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("blue")
         .short("b")
         .long("img-blue")
         .help("Sets the blue amount in the color of output image")
         .value_name("FILE")
         .takes_value(true))
    .arg(Arg::with_name("alpha")
         .short("a")
         .long("img-alpha")
         .help("Sets the alpha amount in the color of the output image")
         .value_name("FILE")
         .takes_value(true))
    .get_matches();

  // Retrieve the command-line arguments
  let verbose = matches.is_present("verbose");
  let iter_num = matches.value_of("iter-num").unwrap_or("1000000").parse::<u32>().unwrap();
  let output_file = matches.value_of("output").unwrap_or("triangle.png");
  let img_width = matches.value_of("img-width").unwrap_or("1024").parse::<u32>().unwrap();
  let img_height = matches.value_of("img-height").unwrap_or("1024").parse::<u32>().unwrap();
  let red = matches.value_of("red").unwrap_or("255").parse::<u8>().unwrap();
  let green = matches.value_of("green").unwrap_or("0").parse::<u8>().unwrap();
  let blue = matches.value_of("blue").unwrap_or("0").parse::<u8>().unwrap();
  let alpha = matches.value_of("alpha").unwrap_or("255").parse::<u8>().unwrap();

  // Print verbose information
  if verbose {
    println!("Number of iterations: {}", iter_num);
    println!("Output file path: {}", output_file);
    println!("Image dimensions: {}X{}", img_width, img_height);
    println!("Image color: [R:{}, G:{}, B:{}, A:{}]", red, green, blue, alpha);
  }

  // Generate the actual triangle
  generate_sierpinski_tri(
    output_file,
    img_width,
    img_height,
    iter_num,
    [red, green, blue, alpha]);
}
